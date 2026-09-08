# Trillionnium World Documentation

This index separates current truth, normative contracts, detailed module design, operational runbooks, machine evidence, and historical provenance. A document is not current merely because it lives under `docs/`.

## 1. Current truth hierarchy

Read in this order:

1. `../PROJECT_BOUNDARY.md` and `../PROJECT_BOUNDARY.json` — binding repository and authority ownership.
2. `../CURRENT_PLAN.md` — operative candidate, execution snapshot, and convergence interpretation.
3. `catalog.json` — machine-checked current document catalogue and review ownership.
4. `status/world-plan-v4-execution-truth-2026-09-02.json` — selected execution snapshot; later explicit live observations in `CURRENT_PLAN.md` take precedence only for their stated scope.
5. `status/CURRENT.md` — generated human-readable release-denominator view.
6. `../GAME_STATUS.md` — native gameplay/runtime evidence and explicit limitations.

When current documents disagree, the binding boundary and accepted ADRs take precedence. A contradiction is a release blocker and must be removed rather than explained away with another status file.

## 2. Architecture and authority decisions

- `adr/0001-realtime-authority-and-match-evidence-ownership.md`
- `adr/0002-transaction-free-external-settlement.md`
- `adr/0003-world-domain-authority-and-nakama-canonical-online.md`
- `architecture/trnm-world-system-context-v1.md`
- `architecture/trnm-world-authority-state-ownership-v1.md`
- `architecture/trnm-settlement-lifecycle-v2.md`
- `architecture/trnm-determinism-and-canonical-json-v1.md`
- `architecture/cex-world-authority-cutover-v1.md`

Core rule: World owns deterministic game-domain behavior and unsigned outcome material; Nakama owns canonical online admission/order/recovery/completion signing; Chain owns ingress/finality; CEX owns wallet/ledger custody; Integration owns exact cross-repository locks and release evidence.

## 3. Module contracts and detailed design

- `modules/README.md` maps every active Cargo member to both its local contract and detailed design.
- `development/trnm-world-module-documentation-matrix-v1.md` is the machine-audited active-workspace coverage matrix.
- `modules/world-authority/README.md` covers the separate seven-crate World-domain cutover candidate.
- `scripts/check-trnm-world-module-documentation.py` checks active local entry contracts.
- `scripts/check-trnm-world-detailed-documentation.py` checks active detailed designs, catalogue, expiry, links and traceability.
- `scripts/check-trnm-world-authority-documentation.py` independently derives and checks the nested cutover workspace.

The eight active game-product designs are under `modules/trnm-*-design.md`. The seven isolated cutover designs are under `modules/world-authority/`. Documentation proves reviewability only; source, tests, hosted execution, server controls, cross-repository compatibility, deployment, custody, human evidence, and release authorization remain separate denominators.

## 4. Current implementation boundary

The active game-product workspace is defined by `../trillionnium/Cargo.toml`. The semantic `trnm-game-server/build.rs` / `src/lib.rs.in` source-generation authority has been retired on the operative Plan V4 candidate: the crate directly compiles Git-tracked ordinary source. Campaign, RTS simulation, and game-server correctness code is currently partitioned into ownership-labelled `lib_parts` and included by small root modules.

That source publication closes the hidden build-time generation defect, but the `include!` partition remains a transitional review layout rather than the final Rust module architecture. The target is true `mod`/trait/visibility boundaries described in `development/trnm-world-module-decomposition-v1.md`; no document should continue to describe the removed generator as current fact.

### World-domain authority cutover candidate

`../trillionnium/crates/world-authority/` is an isolated seven-crate workspace imported from the current World-domain authority candidate together with its PostgreSQL migrations, cutover/provenance contracts, source gates and dedicated read-only workflows. It is not a member of the active game-product workspace and carries no prior workflow/review/production credit merely because the source objects are present.

The nested workspace may replace deterministic World aggregate mutation historically embedded in CEX after exact reconciliation and one fenced writer-epoch transition. It must not replace Nakama canonical online admission/order/recovery/signing or CEX wallet/ledger custody. Fixture adapters and the file-backed server remain test-only; production authorization is `not_granted`.

## 5. Protocols and data contracts

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

Protocol documents must define canonical encoding, resource ceilings, unknown-field behavior, stable machine errors, compatibility/retirement windows, owner boundaries, and positive/negative vectors. Future canonical Nakama APIs are defined by the Nakama owner, not expanded through the World compatibility enclave or cutover server.

## 6. Database, settlement, cutover, and operations

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

Operational documents state scope, preconditions, owner, exact commands, expected outputs, rollback, evidence capture, expiry, and escalation. Chain/BFT/PoUW worker operations are outside this repository and must not reappear in the root World manual.

## 7. Security, testing, and release

- `security/trnm-world-threat-model-v1.md`
- `security/trnm-world-service-identity-secrets-v1.md`
- `development/trnm-world-testing-strategy-v2.md`
- `release/trnm-world-release-gate-matrix-v2.md`
- `release/trnm-world-evidence-record-v1.md`
- `release/trnm-world-reproducible-release-v1.md`

Evidence classes are ordered: source-static, unit, database black box, single-host runtime, cross-repository integration, cross-host, public network, human, custody/security, and commercial/legal. A lower class cannot satisfy a higher class by implication.

## 8. Automated checks

The principal source-side checks include:

- `../scripts/check_trnm_game_product.sh`
- `../scripts/check_trnm_authority_boundary.sh`
- `../scripts/check_trnm_runtime_configuration.sh`
- `../scripts/check_trnm_settlement_outbox_contract.sh`
- `../scripts/check_trnm_settlement_transaction_boundary.sh`
- `../scripts/check-trnm-world-transition-conformance.py`
- `../scripts/check-trnm-world-module-documentation.py`
- `../scripts/check-trnm-world-detailed-documentation.py`
- `../scripts/check-trnm-world-authority-documentation.py`
- `../scripts/check-trnm-world-authority-cutover.sh`
- `../scripts/check-trnm-world-authority-postgres-installation.sh`
- `../scripts/run-trnm-world-authority-postgres-check.sh`
- `../scripts/check-trnm-world-documentation.py`

Dedicated read-only workflows are `.github/workflows/trnm-world-module-documentation.yml`, `trnm-world-authority-cutover.yml`, and `trnm-world-postgres-cutover.yml`. A workflow definition is not execution. At the time of this convergence update, repository-native World workflow scheduling remains a server/organization configuration blocker. No source document may manufacture a run, job, step, log, artifact, required context, or independent approval.

## 9. Historical material

Legacy Chain, PoUW, ZKP-platform, old Web4, removed World/Bevy, Android, and superseded planning/status material belongs under `archive/` or an explicitly historical directory. It may be cited for provenance, but it cannot define current architecture, development commands, or release posture. New current documents must be registered in `catalog.json`; superseded documents must be marked or moved rather than left as competing truth sources.
