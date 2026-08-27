# Trillionnium World Documentation Index

## Current truth sources

- Current development plan: `development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md`
- Machine-readable execution manifest: `development/trillionnium-world-development-plan-2026-08-27.json`
- Product status and honest open gates: `../GAME_STATUS.md`
- Repository and authority boundary: `../PROJECT_BOUNDARY.md`
- Authority ownership decision: `adr/0001-realtime-authority-and-match-evidence-ownership.md`
- External-settlement transaction decision: `adr/0002-transaction-free-external-settlement.md`
- World match-evidence boundary: `protocol/trnm-match-evidence-commitment-v1.md`
- Settlement outbox design: `development/trnm-settlement-outbox-v1.md`
- Runtime configuration: `operations/trnm-game-server-runtime-configuration.md`
- Native release gates: `development/trnm-native-game-release-gates-v1.md`
- Current RPG/RTS product contract: `development/trillionnium-rpg-rts-closed-loop-v1.md`
- Current native game/CEX economy contract: `development/trnm-cex-economy-integration-v1.md`
- Current native UI vertical slice: `development/trnm-world-ui-vertical-slice-v1.md`
- Machine-readable UI acceptance matrix: `development/trnm-world-ui-acceptance-v1.json`

## Current implementation references

- Native client: `../trillionnium/crates/trnm-first-contact`
- Native player control surface: `../trillionnium/crates/trnm-first-contact/src/ui`
- Deterministic RTS simulation: `../trillionnium/crates/trnm-rts-sim`
- Campaign/save/progression authority: `../trillionnium/crates/trnm-campaign-core`
- Economy intent/receipt vocabulary: `../trillionnium/crates/trnm-economy-protocol`
- Online wire compatibility: `../trillionnium/crates/trnm-online-protocol`
- World-local compatibility authority enclave: `../trillionnium/crates/trnm-game-server`
- Settlement outbox invariant contract: `../trillionnium/tools/trnm-settlement-outbox-contract`

## Current checks

- Product workspace boundary: `../scripts/check_trnm_game_product.sh`
- Authority ownership boundary: `../scripts/check_trnm_authority_boundary.sh`
- Runtime configuration boundary: `../scripts/check_trnm_runtime_configuration.sh`
- Settlement outbox contract: `../scripts/check_trnm_settlement_outbox_contract.sh`
- Legacy settlement-debt upper bound: `../scripts/check_trnm_settlement_transaction_boundary.sh`
- Native UI architecture and honest-authority contract: `../scripts/check-trnm-ui-contract.sh`
- Native UI forbidden-claim negative fixtures: `../scripts/test-trnm-ui-contract-negative.sh`

## Historical material

Historical Chain, Web4, World/Bevy, Android, public-launch and old release-review
material remains under `archive/` or legacy directories for provenance. It is
not a current architecture, product gate or release truth source unless a
current document explicitly cites a bounded artifact from it.

Documents under `architecture/`, `protocol/`, `runbooks/` or `release/` that
primarily describe the former Rust L1/shared repository should be migrated to
the owning repository or moved under `archive/legacy-chain/`. Their directory
name alone does not make them current World documentation.
