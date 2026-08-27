# Trillionnium World Documentation Index

## Current truth sources

- Current development plan: `development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-27.md`
- Machine-readable execution manifest: `development/trillionnium-world-development-plan-2026-08-27.json`
- Product status and honest open gates: `../GAME_STATUS.md`
- Repository and authority boundary: `../PROJECT_BOUNDARY.md`
- Authority ownership decision: `adr/0001-realtime-authority-and-match-evidence-ownership.md`
- External-settlement transaction decision: `adr/0002-transaction-free-external-settlement.md`
- World match-evidence boundary: `protocol/trnm-match-evidence-commitment-v1.md`
- World deterministic runtime protocol: `protocol/trnm-world-runtime-v1.md`
- World → Nakama shadow program: `development/trnm-world-nakama-shadow-v1.md`
- Authority cutover and rollback runbook: `runbooks/trnm-world-authority-cutover-v1.md`
- Settlement outbox design: `development/trnm-settlement-outbox-v1.md`
- Runtime configuration: `operations/trnm-game-server-runtime-configuration.md`
- Native release gates: `development/trnm-native-game-release-gates-v1.md`
- Current RPG/RTS product contract: `development/trillionnium-rpg-rts-closed-loop-v1.md`
- Current native game/CEX economy contract: `development/trnm-cex-economy-integration-v1.md`

## Current contract references

- Runtime request/result schema: `../contracts/world-runtime/v1/trnm-world-runtime-v1.schema.json`
- Runtime observation/shadow schema: `../contracts/world-runtime/v1/trnm-world-shadow-v1.schema.json`
- Runtime canonical vectors: `../contracts/world-runtime/v1/golden-vectors.json`
- Shadow comparison vectors: `../contracts/world-runtime/v1/shadow-vectors.json`
- Stable runtime/error catalogue: `../contracts/world-runtime/v1/error-catalog.json`
- Authority compatibility matrix: `../contracts/world-runtime/v1/compatibility-matrix.json`

## Current implementation references

- Native client: `../trillionnium/crates/trnm-first-contact`
- Deterministic RTS simulation: `../trillionnium/crates/trnm-rts-sim`
- Campaign/save/progression authority: `../trillionnium/crates/trnm-campaign-core`
- Economy intent/receipt vocabulary: `../trillionnium/crates/trnm-economy-protocol`
- Online wire compatibility: `../trillionnium/crates/trnm-online-protocol`
- World-local compatibility authority enclave: `../trillionnium/crates/trnm-game-server`
- Settlement outbox invariant contract: `../trillionnium/tools/trnm-settlement-outbox-contract`
- Bevy-free deterministic runtime adapter: `../contracts/world-runtime/rust`
- Runtime execution host and shadow comparator: `../contracts/world-runtime/host`

## Current checks

- Product workspace boundary: `../scripts/check_trnm_game_product.sh`
- Authority ownership boundary: `../scripts/check_trnm_authority_boundary.sh`
- Runtime configuration boundary: `../scripts/check_trnm_runtime_configuration.sh`
- Settlement outbox contract: `../scripts/check_trnm_settlement_outbox_contract.sh`
- Legacy settlement-debt upper bound: `../scripts/check_trnm_settlement_transaction_boundary.sh`
- Runtime canonical-vector verifier: `../scripts/verify-trnm-world-runtime-v1.py`
- Independent shadow-vector verifier: `../scripts/verify-trnm-world-shadow-v1.py`
- Runtime/host authority boundary: `../scripts/check-trnm-world-runtime-boundary.sh`
- Runtime authority negative fixtures: `../scripts/test-trnm-world-runtime-boundary-negative.sh`
- Exact runtime source manifest: `../scripts/emit-trnm-world-runtime-v1-source-manifest.py`

## Historical material

Historical Chain, Web4, World/Bevy, Android, public-launch and old release-review
material remains under `archive/` or legacy directories for provenance. It is
not a current architecture, product gate or release truth source unless a
current document explicitly cites a bounded artifact from it.

Documents under `architecture/`, `protocol/`, `runbooks/` or `release/` that
primarily describe the former Rust L1/shared repository should be migrated to
the owning repository or moved under `archive/legacy-chain/`. Their directory
name alone does not make them current World documentation.
