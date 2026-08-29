# Trillionnium World

Trillionnium World is the game-product repository for the native RPG/RTS client,
deterministic World rules and simulation, campaign/progression behavior,
player-facing economy intents, and World-owned replay/outcome material.

## Current posture

- Engineering: **technical alpha**.
- Player-facing product: **pre-alpha**.
- Public online, public player market, custody, and commercial release: **NO-GO**.
- Current executable plan: [`CURRENT_PLAN.md`](CURRENT_PLAN.md).
- Current machine-readable gap registry:
  [`docs/status/world-gap-registry-v2.json`](docs/status/world-gap-registry-v2.json).

Source implementation, automated validation, deployed evidence, human evidence,
and commercial approval are separate denominators. A source-complete row does
not grant production or public-release credit.

## Authority boundary

| Responsibility | Accountable system |
| --- | --- |
| Authored content, deterministic rules/simulation, World outcome hash, unsigned replay material, native client | Trillionnium World |
| Online admission, canonical command order, online idempotency, restart recovery, archive roots, `MatchCompletedV1` signing | Trillionnium Nakama |
| Consensus, inclusion and finality | Trillionnium Chain |
| Wallet/ledger settlement and custody | CEX |
| Exact cross-repository component locks and release evidence | Trillionnium Integration |

`trillionnium/crates/trnm-game-server` remains a bounded compatibility enclave
for migration and rollback evidence. It is not the target public online
authority and must not acquire a second canonical ordering/root/signature.

## Active workspace

```text
trillionnium/
  crates/
    trnm-first-contact       native Bevy client
    trnm-campaign-core       campaign/save/progression aggregate
    trnm-economy-protocol    game-owned intent/receipt contract
    trnm-rpg-core            RPG vocabulary
    trnm-rts-protocol        deterministic order contract
    trnm-rts-sim             Bevy-free deterministic RTS simulation
    trnm-online-protocol     compatibility wire types
    trnm-game-server         compatibility online enclave and settlement services
assets/first_contact/        authored maps, atlases and audio
config/                      runtime configuration examples
deploy/systemd/              rendered service templates
docs/                        current architecture, plans, protocols and evidence rules
scripts/                     repository gates, packaging and operator entry points
```

The excluded `trillionnium/crates/platform` tree is legacy migration material
and is not part of the active game workspace.

## Development bootstrap

```bash
git clone https://github.com/TrillionniumFoundation/Trillionnium-World.git
cd Trillionnium-World
git switch -c fix/world-<topic>
bash scripts/project-preflight.sh

cd trillionnium
cargo metadata --locked --no-deps --format-version 1 >/dev/null
cargo fmt --all -- --check
cargo test --workspace --all-targets --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
```

Repository boundary and product gates:

```bash
./scripts/check_trnm_game_product.sh
./scripts/check_trnm_authority_boundary.sh
./scripts/check_trnm_runtime_configuration.sh
./scripts/check_trnm_settlement_outbox_contract.sh
./scripts/check_trnm_settlement_transaction_boundary.sh
```

PostgreSQL-backed settlement tests require the explicit test database contract
documented in the current settlement runbook. Empty, skipped, cancelled or
unbound checks are blockers, not passes.

## Documentation map

- [Current plan](docs/development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md)
- [Current status](docs/status/CURRENT.md)
- [System architecture](docs/architecture/trnm-world-system-architecture-v1.md)
- [Authority ADR](docs/adr/0001-realtime-authority-and-match-evidence-ownership.md)
- [Settlement ADR](docs/adr/0002-transaction-free-external-settlement.md)
- [Reviewable source and CI ADR](docs/adr/0003-reviewable-source-and-non-self-modifying-ci.md)
- [Module-decomposition plan](docs/development/trnm-world-module-decomposition-v1.md)
- [Protocol and database contract plan](docs/development/trnm-world-protocol-and-database-contract-plan-v1.md)
- [Threat model](docs/security/trnm-world-threat-model-v1.md)
- [Release evidence contract](docs/release/trnm-world-release-evidence-contract-v1.md)
- [Gap-closure runbook](docs/runbooks/trnm-world-gap-closure-operations-v1.md)

Historical Chain, Web4, legacy World/Bevy, and superseded plan material may be
retained for provenance, but it is not current product truth unless explicitly
linked from the current documentation index.

## Contribution and merge discipline

- Work on one lane-prefixed branch and one reviewable PR.
- Never develop directly on `main`.
- Do not merge your own PR.
- CI may validate and upload artifacts; it must not rewrite source or publish a
  verified tag for its own unreviewed modifications.
- Every release claim must bind the exact commit, Git tree, binaries, toolchain,
  environment, raw evidence hashes, limitations, reviewer and expiry.
