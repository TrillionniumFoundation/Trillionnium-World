# Trillionnium World

Trillionnium World is the game-product repository for the Trillionnium RPG/RTS experience. It owns authored game content, deterministic RPG/RTS rules, campaign/save/progression behavior, the native client, player-facing economic intents, and unsigned replay/outcome material.

It does not own canonical online admission or event ordering, Chain finality, wallet/ledger custody, or cross-repository release authorization. Those responsibilities remain with the accountable Nakama, Chain, CEX, and Integration systems.

## Authoritative entry points

Read these files before interpreting implementation or release claims:

1. [`PROJECT_BOUNDARY.md`](PROJECT_BOUNDARY.md) — repository and authority boundary.
2. [`GAME_STATUS.md`](GAME_STATUS.md) — native game-product status and open evidence.
3. [`RELEASE_READINESS.md`](RELEASE_READINESS.md) — repository-level release decision.
4. [`docs/README.md`](docs/README.md) — documentation map and historical/current separation.
5. [`trillionnium/Cargo.toml`](trillionnium/Cargo.toml) — active game-product workspace.

Historical material, local fixtures, generated status text, or a green subproject check cannot grant public-online, custody, player-market, commercial, or production authorization.

## Development validation

Development must use pull requests against protected `main`; do not develop directly on `main`.

```bash
bash scripts/check_trnm_game_product.sh

cd trillionnium
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

The repository-native admission workflow validates the exact pull-request head and GitHub prospective merge using read-only credentials. A missing, empty, skipped, cancelled, stale, or identity-unbound run is a blocker rather than a pass.

## Release posture

The release state is defined only by `RELEASE_READINESS.md` and accepted evidence for the exact candidate. Source presence and documentation completeness do not imply production readiness.
