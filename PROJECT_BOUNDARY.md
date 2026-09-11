# Trillionnium World Repository Boundary

- Project ID: `trillionnium-world`
- Canonical repository root: the directory returned by `git rev-parse --show-toplevel`
- Primary lane: `game-product`
- Canonical remote: `TrillionniumFoundation/Trillionnium-World`
- Repository-wide documentation root: `docs/`

## Owned by the game-product lane

The game-product lane owns native gameplay and presentation; RPG/world vocabulary; campaign, save, progression, and settlement authority; deterministic RTS order and simulation contracts; game-server behavior and online match authority; player-facing economy intent/receipt behavior; authored game content; game packaging; and match/replay evidence.

The canonical game workspace is `trillionnium/Cargo.toml`. Its members are the eight game crates recorded in `docs/modules.json`.

## Independent platform workspace

`trillionnium/crates/platform/` is an independently built chain/node/PoCO platform workspace retained while repository separation is completed.

It is not a dependency of the native player game and must be built with an explicit manifest path. Platform tests, benchmarks, or release evidence cannot be cited as gameplay evidence. New game-product dependencies on the platform tree are forbidden.

Any remaining cross-workspace source or path dependency is migration debt. Replace it with a versioned protocol crate, a generated schema/client artifact, or an external service contract with explicit failure semantics.

## External-contract perimeter

`contracts/` owns Rust MVP contract semantics and normalized audit-event shapes. It does not currently prove a production Host ABI, deterministic WASM executor integration, canonical `wasm32-unknown-unknown` artifacts, or public-mainnet readiness.

## Web4 boundary

`web4-frontend/` is a read-only query client with explicit mock fallback. It does not own native gameplay, online match authority, chain state, a write-enabled dashboard backend, or repository-wide release readiness.

## Not owned by the game-product lane

The game-product lane does not own public-chain consensus, validator lifecycle, peer discovery, state sync, production wallet custody, HSM/KMS operations, public-mainnet economics, cross-repository control-plane services, third-party runtime infrastructure, or public multi-region orchestration unless scope is changed by an approved architecture decision.

## Change-control rules

A pull request that changes a boundary must include an Architecture Decision Record, updates to `docs/modules.json`, dependency and threat-model impact, compatibility and migration plan, rollback path, and exact verification commands with evidence scope.

No document may use a developer-specific absolute workstation path as a canonical repository path.
