# Trillionnium World Documentation

This index separates current truth, normative contracts, module designs, runbooks, machine evidence and historical provenance. A document is not current merely because it lives under `docs/`.

## Current truth hierarchy

Read in this order:

1. `../PROJECT_BOUNDARY.md` and `../PROJECT_BOUNDARY.json`;
2. `../CURRENT_PLAN.md`;
3. `development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md` and its current convergence update;
4. `development/trnm-world-gap-closure-ledger-v5.json`;
5. `status/world-plan-v4-execution-truth-2026-09-02.json` plus later explicit live observations in `CURRENT_PLAN.md`;
6. `status/CURRENT.md`;
7. `../GAME_STATUS.md` and `../RELEASE_READINESS.md`.

Accepted ADRs and binding ownership rules outrank planning prose. A contradiction is a blocker and must be removed rather than hidden by adding another status document.

## Architecture and ADRs

- `adr/0001-realtime-authority-and-match-evidence-ownership.md`
- `adr/0002-transaction-free-external-settlement.md`
- `adr/0003-world-authority-service-boundary.md`
- `architecture/trnm-world-system-context-v1.md`
- `architecture/trnm-world-authority-state-ownership-v1.md`
- `architecture/trnm-settlement-lifecycle-v2.md`
- `architecture/trnm-determinism-and-canonical-json-v1.md`

Core rule: World owns deterministic game-domain behavior; Nakama owns canonical online admission/order/completion; Chain owns finality; CEX owns wallet/ledger custody; Integration owns exact multi-repository locks and release evidence.

## Module designs

`modules/README.md` is the entry point for all eight active crates. Each module has a local summary README plus a detailed design covering interfaces, state, dependency direction, concurrency/cancellation, durability/recovery, retry/idempotency, versioning, budgets, observability/security, evidence and open work. The exact inventory is `modules/contracts-v2.json`.

The documentation gate checks presence, one-to-one workspace coverage, mandatory sections, local links, stale source-generation claims, personal paths and authority/release overclaims. It still does not prove implementation conformance or release eligibility.

## Protocol and data contracts

- `protocol/trnm-world-transition-v1.md`
- `protocol/trnm-rts-order-intake-v1.md`
- `protocol/trnm-rts-intake-differential-v1.md`
- `protocol/trnm-match-evidence-commitment-v1.md`
- `protocol/trnm-settlement-receipt-recovery-v1.md`
- `database/trnm-world-postgres-contract-v1.md`

Protocols define canonical encoding, limits, unknown-field behavior, stable errors, compatibility/retirement and negative vectors. Cross-repository authority is never inferred from a shared type name.

## Development and testing

- `development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md`
- `development/TRILLIONNIUM_WORLD_CLOSURE_EXECUTION_BOARD_V5.md`
- `development/trnm-world-module-decomposition-v1.md`
- `development/trnm-world-testing-strategy-v2.md`
- `development/trnm-world-module-documentation-matrix-v1.md`

Source implementation has removed semantic game-server `build.rs`/`src/lib.rs.in` authority on the operative candidate. The ordinary `lib_parts` layout is reviewable but remains an intermediate decomposition; true Rust module/trait boundaries are still required where listed by the module designs.

## Security, operations and release

- `security/trnm-world-threat-model-v1.md`
- `security/trnm-world-service-identity-secrets-v1.md`
- `operations/trnm-game-server-runtime-configuration.md`
- `runbooks/trnm-settlement-operations-v1.md`
- `runbooks/trnm-settlement-shutdown-and-quarantine-v1.md`
- `runbooks/trnm-authority-cutover-rollback-v1.md`
- `release/trnm-world-release-gate-matrix-v2.md`
- `release/trnm-world-evidence-record-v1.md`
- `../OPERATIONS.md`
- `../RELEASE_READINESS.md`

Runbooks state scope, prerequisites, exact commands, expected output, rollback, evidence capture, expiry and escalation. Templates and local fixtures do not constitute deployed evidence.

## Current automated checks

- `../scripts/check-trnm-world-module-documentation.py`
- `../scripts/test-trnm-world-module-documentation-negative.py`
- `../scripts/check-trnm-world-documentation.py`
- `../scripts/check-trnm-world-transition-conformance.py`
- `../scripts/check-trnm-world-ci-integrity.py`
- `../scripts/check_trnm_game_product.sh`
- `../scripts/check_trnm_authority_boundary.sh`
- `../scripts/check_trnm_runtime_configuration.sh`
- `../scripts/check_trnm_settlement_outbox_contract.sh`
- `../scripts/check_trnm_settlement_transaction_boundary.sh`

Workflow definitions are only source. Missing or empty exact-head runs receive no execution credit.

## Historical material

Legacy Chain, PoUW, old Web4, retired World/Bevy, Android and superseded Plan branches are provenance only. Historical files should be under `archive/` or carry explicit historical metadata. They cannot define current commands, authority, release posture or production claims.
