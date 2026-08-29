# Trillionnium World Documentation

This index separates **current truth**, **normative contracts**, **operational runbooks**, **machine evidence**, and **historical provenance**. A document is not current merely because it lives under `docs/`.

## 1. Current truth hierarchy

Read in this order:

1. `../PROJECT_BOUNDARY.md` — binding repository and authority ownership.
2. `../CURRENT_PLAN.md` — pointer to the active executable plan.
3. `development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_2026-08-29.md` — ordered workstreams, exit gates and stop conditions.
4. `development/trillionnium-world-development-plan-2026-08-29.json` — machine-readable plan.
5. `development/trnm-world-gap-closure-ledger-v4.json` — exact gaps, owners, dependencies and evidence classes.
6. `status/world-gates-v1.json` and `status/CURRENT.md` — release-denominator posture.
7. `../GAME_STATUS.md` — native gameplay/runtime evidence and explicit limitations.

When two current documents disagree, the binding boundary and accepted ADRs take precedence; the contradiction is itself a release blocker.

## 2. Architecture and ADRs

- `adr/0001-realtime-authority-and-match-evidence-ownership.md`
- `adr/0002-transaction-free-external-settlement.md`
- `architecture/trnm-world-system-context-v1.md`
- `architecture/trnm-world-authority-state-ownership-v1.md`
- `architecture/trnm-settlement-lifecycle-v2.md`
- `architecture/trnm-determinism-and-canonical-json-v1.md`

Core ownership rule:

- World: deterministic game rules, content, World state transitions, outcome hashes and unsigned game-domain material.
- Nakama: online admission, canonical total order, command idempotency, restart recovery, canonical archive roots and completion signing.
- Chain: ingress, consensus, inclusion and finality.
- CEX: wallet/ledger settlement and custody.
- Integration: exact cross-repository locks, compatibility matrices and release evidence.

## 3. Protocols and data contracts

- `protocol/trnm-world-transition-v1.md`
- `protocol/schemas/trnm-world-transition-v1.schema.json`
- `protocol/vectors/trnm-world-transition-v1.json`
- `protocol/vectors/trnm-world-transition-negative-v1.json`
- `protocol/trnm-match-evidence-commitment-v1.md`
- `protocol/trnm-settlement-receipt-recovery-v1.md`
- `database/trnm-world-postgres-contract-v1.md`

Protocol documents must define canonical encoding, resource budgets, unknown-field behavior, compatibility windows, stable machine errors, owner boundaries and negative vectors.

## 4. Development and implementation

- `development/trillionnium-rpg-rts-closed-loop-v1.md`
- `development/trnm-native-game-release-gates-v1.md`
- `development/trnm-settlement-outbox-v1.md`
- `development/trnm-settlement-database-contract-v1.md`
- `development/trnm-world-nakama-authority-migration-v1.md`
- `development/trnm-world-module-decomposition-v1.md`
- `development/trnm-world-testing-strategy-v2.md`

Active code references:

- native client: `../trillionnium/crates/trnm-first-contact`
- campaign/save/progression: `../trillionnium/crates/trnm-campaign-core`
- deterministic RTS: `../trillionnium/crates/trnm-rts-sim`
- game economy vocabulary: `../trillionnium/crates/trnm-economy-protocol`
- online compatibility wire types: `../trillionnium/crates/trnm-online-protocol`
- World-local compatibility server: `../trillionnium/crates/trnm-game-server`
- deterministic World transition contract: `../trillionnium/contracts/trnm-world-transition-v1`

## 5. Security, operations and release

- `security/trnm-world-threat-model-v1.md`
- `operations/trnm-game-server-runtime-configuration.md`
- `runbooks/trnm-settlement-operations-v1.md`
- `runbooks/trnm-settlement-shutdown-and-quarantine-v1.md`
- `runbooks/trnm-authority-cutover-rollback-v1.md`
- `release/trnm-world-release-gate-matrix-v2.md`
- `release/trnm-world-evidence-record-v1.md`

Operational documents must state scope, preconditions, owner, exact commands, expected outputs, rollback, evidence capture, expiry and escalation. Templates or source tests do not constitute production evidence.

## 6. Current automated checks

- `../scripts/check_trnm_game_product.sh`
- `../scripts/check_trnm_authority_boundary.sh`
- `../scripts/check_trnm_runtime_configuration.sh`
- `../scripts/check_trnm_settlement_outbox_contract.sh`
- `../scripts/check_trnm_settlement_transaction_boundary.sh`
- `../scripts/check-trnm-settlement-runtime-status.py`
- `../scripts/check-trnm-world-transition-conformance.py`
- `../scripts/check-trnm-world-documentation.py`

CI workflows must have read-only repository permissions. They may upload immutable evidence artifacts, but may not modify, commit, push, tag or promote candidate source.

## 7. Evidence classes

Every claim is assigned exactly one evidence class:

- `source_static`
- `unit`
- `database_black_box`
- `single_host_runtime`
- `cross_repository_integration`
- `cross_host`
- `public_network`
- `human`
- `custody_security`
- `commercial_legal`

A lower class can never satisfy a higher-class gate by implication.

## 8. Historical material

Legacy Chain, PoUW, Web4, World/Bevy, Android and old public-launch material belongs under `archive/` or an explicitly marked historical directory. It may be cited for provenance, but it cannot define current architecture, current release posture or current development commands.

Current documents use front matter or an explicit status block containing owner, status, applicability, review date and supersession information. Missing metadata is documentation debt and cannot silently create normative truth.