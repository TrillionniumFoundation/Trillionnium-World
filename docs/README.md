# Trillionnium World Documentation

This index separates current truth, normative contracts, detailed module design, operational runbooks, machine evidence and historical provenance. A document is not current merely because it lives under `docs/`.

## 1. Current truth hierarchy

Read in this order:

1. `../PROJECT_BOUNDARY.md` and `../PROJECT_BOUNDARY.json` — binding repository and authority ownership.
2. `../CURRENT_PLAN.md` — operative PR/branch, selected execution snapshot and live convergence interpretation.
3. `catalog.json` — strict machine current-document catalogue and review ownership.
4. `status/world-plan-v4-execution-truth-2026-09-08.json` — selected recorded execution snapshot.
5. `status/CURRENT.md` — deterministic human-readable rendering of that snapshot.
6. `../RELEASE_READINESS.md` — repository-level release decision and no-credit boundary.
7. `development/trnm-world-gap-closure-ledger-v6.json` — current denominator/owner/state ledger.
8. `integration/trnm-world-cex-current-pending-lock-v2.json` — coordination-only current CEX candidate observation.
9. `../GAME_STATUS.md` — native gameplay/runtime evidence and explicit product limitations.

When current documents disagree, the binding boundary and accepted ADRs take precedence. A contradiction is a blocker and must be removed rather than explained away with another status file.

Historical Plan V4 inputs remain discoverable but do not select the current candidate:

- `development/TRILLIONNIUM_WORLD_PLAN_V4_CONVERGENCE_ADDENDUM_2026-08-30.md`
- `development/TRILLIONNIUM_WORLD_PLAN_V4_EXECUTION_UPDATE_2026-08-30.md`
- `status/world-v4-convergence-state-2026-08-30.json`
- `development/trillionnium-world-development-plan-2026-08-29.json`
- `development/trnm-world-gap-closure-ledger-v4.json`
- `development/trnm-world-gap-closure-ledger-v5.json`
- `integration/trnm-world-cex-sequence-50-pending-lock-v1.json`

The V5 ledger and sequence-50 CEX lock are historical coordination records. They must not be interpreted as current PR/component selection.

## 2. Architecture and authority decisions

- `adr/0001-realtime-authority-and-match-evidence-ownership.md`
- `adr/0002-transaction-free-external-settlement.md`
- `adr/0003-world-domain-authority-and-nakama-canonical-online.md`
- `architecture/trnm-world-system-context-v1.md`
- `architecture/trnm-world-authority-state-ownership-v1.md`
- `architecture/trnm-settlement-lifecycle-v2.md`
- `architecture/trnm-determinism-and-canonical-json-v1.md`
- `architecture/cex-world-authority-cutover-v1.md`

Core rule: World owns deterministic game-domain behavior and World aggregate mutation under one fenced writer epoch; Nakama owns canonical online admission/order/recovery/completion signing; CEX owns wallet/ledger custody; Chain owns ingress/finality; Integration owns exact cross-repository locks and release evidence.

## 3. Module contracts and detailed design

`modules/README.md` distinguishes:

- the eight active Cargo members in `trillionnium/Cargo.toml`;
- the isolated seven-crate World-domain authority candidate under `trillionnium/crates/world-authority/`.

The active designs are `modules/trnm-*-design.md`. The cutover designs are under `modules/world-authority/`.

Executable documentation gates:

```bash
python3 scripts/check-trnm-world-module-documentation.py
python3 scripts/test-trnm-world-module-documentation-negative.py
python3 scripts/check-trnm-world-detailed-documentation.py
python3 scripts/test-trnm-world-detailed-documentation.py
python3 scripts/check-trnm-world-authority-documentation.py
python3 scripts/test-trnm-world-authority-documentation.py
python3 scripts/check-trnm-world-active-closure.py
python3 scripts/test-trnm-world-active-closure.py
```

A documentation pass proves reviewability only. It cannot prove implementation conformance, hosted CI, server controls, deployment, custody, human evidence or release authorization.

## 4. Current implementation boundary

The semantic `trnm-game-server/build.rs` / `src/lib.rs.in` source authority is retired on the current candidate. The active campaign, RTS simulation and compatibility-server correctness code is directly compiled Git-tracked source, currently partitioned through ownership-labelled `lib_parts`.

That removes hidden build-time semantics but does not make `include!` the final architecture. The target remains true Rust modules, traits and visibility boundaries under `development/trnm-world-module-decomposition-v1.md`.

The isolated World-domain authority workspace is not silently part of the active eight-crate product denominator. It contains fixture adapters and a file-backed development server only; production authorization remains `not_granted`.

## 5. Protocols and data contracts

Primary current contracts include:

- `protocol/trnm-world-transition-v1.md`
- `protocol/schemas/trnm-world-transition-v1.schema.json`
- `protocol/vectors/trnm-world-transition-v1.json`
- `protocol/vectors/trnm-world-transition-negative-v1.json`
- `protocol/trnm-rts-order-intake-v1.md`
- `protocol/trnm-rts-intake-differential-v1.md`
- `protocol/trnm-match-evidence-commitment-v1.md`
- `protocol/trnm-settlement-receipt-recovery-v1.md`
- `protocol/trnm-world-http-api-v1.md`
- `protocol/trnm-world-websocket-v1.md`
- `protocol/trnm-world-error-catalog-v1.md`
- `protocol/trnm-world-compatibility-matrix-v1.md`
- `contracts/trillionnium-world-authority-cutover-v1.json`
- `contracts/trillionnium-world-authority-provenance-v1.json`

Protocol documents define canonical encoding, resource ceilings, unknown-field behavior, stable errors, compatibility/retirement windows, authority boundaries and positive/negative vectors. Future canonical Nakama APIs are owned in the Nakama repository.

## 6. Database, settlement, cutover and operations

- `database/trnm-world-postgres-contract-v1.md`
- `database/trnm-world-stored-procedure-catalog-v1.md`
- `database/trnm-world-lock-order-v1.md`
- `development/trnm-settlement-outbox-v1.md`
- `development/trnm-settlement-database-contract-v1.md`
- `operations/trnm-game-server-runtime-configuration.md`
- `operations/trnm-world-observability-slo-v1.md`
- `operations/trnm-world-backup-pitr-v1.md`
- `runbooks/trnm-settlement-operations-v1.md`
- `runbooks/trnm-settlement-shutdown-and-quarantine-v1.md`
- `runbooks/trnm-authority-cutover-rollback-v1.md`
- `../deploy/postgres/trnm-world-authority-cutover-v1.sql`
- `../deploy/postgres/trnm-world-authority-cutover-v1-hardening.sql`
- `../deploy/postgres/trnm-world-authority-catalog-fingerprint-v1.sql`
- `../OPERATIONS.md`

Chain/BFT/PoUW worker operations are outside this repository.

## 7. Security, testing and release

- `security/trnm-world-threat-model-v1.md`
- `security/trnm-world-service-identity-secrets-v1.md`
- `development/trnm-world-testing-strategy-v2.md`
- `release/trnm-world-release-gate-matrix-v2.md`
- `release/trnm-world-evidence-record-v1.md`
- `release/trnm-world-reproducible-release-v1.md`
- `../RELEASE_READINESS.md`

Evidence classes are ordered: source-static, unit, database black box, single-host runtime, cross-repository integration, cross-host, public network, human, custody/security and commercial/legal. A lower class cannot satisfy a higher class by implication.

## 8. Automated checks

The principal gates include:

- `../scripts/check_trnm_game_product.sh`
- `../scripts/check_trnm_authority_boundary.sh`
- `../scripts/check_trnm_runtime_configuration.sh`
- `../scripts/check_trnm_settlement_outbox_contract.sh`
- `../scripts/check_trnm_settlement_transaction_boundary.sh`
- `../scripts/check-trnm-world-transition-conformance.py`
- `../scripts/check-trnm-world-ci-integrity.py`
- `../scripts/check-trnm-world-documentation.py`
- `../scripts/check-trnm-world-active-closure.py`
- `../scripts/check-trnm-world-cex-pending-lock.py`
- `../scripts/check-trnm-world-authority-cutover.sh`
- `../scripts/check-trnm-world-authority-postgres-installation.sh`
- `../scripts/run-trnm-world-authority-postgres-check.sh`

A workflow definition is not execution. Repository-native scheduling remains a GitHub repository/organization control-plane blocker until non-empty exact-head and prospective-merge runs exist. No source document may manufacture a run, status or protected context.

## 9. Historical material

Legacy Chain, PoUW, ZKP-platform, old Web4, removed World/Bevy, Android and superseded planning/status material belongs under `archive/` or an explicitly historical directory. It may be cited for provenance but cannot define current architecture, commands or release posture. New current documents must be registered in `catalog.json`; superseded material must not compete with the selected snapshot.
