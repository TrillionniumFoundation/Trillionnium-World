# Trillionnium World Documentation

This index lists current product truth. Historical Chain, Web4, legacy
World/Bevy and superseded plans are not active architecture unless explicitly
linked below.

## Start here

1. [`../PROJECT_BOUNDARY.md`](../PROJECT_BOUNDARY.md)
2. [`../CURRENT_PLAN.md`](../CURRENT_PLAN.md)
3. [`development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md`](development/TRILLIONNIUM_WORLD_DEVELOPMENT_PLAN_V4_2026-08-29.md)
4. [`status/CURRENT.md`](status/CURRENT.md)
5. [`status/world-gap-registry-v2.json`](status/world-gap-registry-v2.json)

## Architecture and decisions

- [`architecture/trnm-world-system-architecture-v1.md`](architecture/trnm-world-system-architecture-v1.md)
- [`adr/0001-realtime-authority-and-match-evidence-ownership.md`](adr/0001-realtime-authority-and-match-evidence-ownership.md)
- [`adr/0002-transaction-free-external-settlement.md`](adr/0002-transaction-free-external-settlement.md)
- [`adr/0003-reviewable-source-and-non-self-modifying-ci.md`](adr/0003-reviewable-source-and-non-self-modifying-ci.md)

## Development and correctness

- [`development/trnm-world-module-decomposition-v1.md`](development/trnm-world-module-decomposition-v1.md)
- [`development/trnm-world-protocol-and-database-contract-plan-v1.md`](development/trnm-world-protocol-and-database-contract-plan-v1.md)
- [`development/trnm-settlement-outbox-v1.md`](development/trnm-settlement-outbox-v1.md)
- [`development/trnm-settlement-fault-evidence-plan-v1.md`](development/trnm-settlement-fault-evidence-plan-v1.md)
- [`development/trnm-world-nakama-authority-migration-v1.md`](development/trnm-world-nakama-authority-migration-v1.md)

## Protocols

- [`protocol/trnm-match-evidence-commitment-v1.md`](protocol/trnm-match-evidence-commitment-v1.md)
- [`protocol/trnm-settlement-receipt-recovery-v1.md`](protocol/trnm-settlement-receipt-recovery-v1.md)
- World transition v1 remains on its isolated contract PR until exact-head
  validation and review are complete.

## Security, operations and release

- [`security/trnm-world-threat-model-v1.md`](security/trnm-world-threat-model-v1.md)
- [`operations/trnm-game-server-runtime-configuration.md`](operations/trnm-game-server-runtime-configuration.md)
- [`runbooks/trnm-settlement-operations-v1.md`](runbooks/trnm-settlement-operations-v1.md)
- [`runbooks/trnm-world-gap-closure-operations-v1.md`](runbooks/trnm-world-gap-closure-operations-v1.md)
- [`release/trnm-world-release-evidence-contract-v1.md`](release/trnm-world-release-evidence-contract-v1.md)

## Evidence classification

Every claim must be classified as one of:

- source/static;
- unit/property;
- local black-box;
- deployed single-host;
- deployed cross-host/public-network;
- human;
- custody/security approval;
- commercial/legal approval.

A stronger denominator cannot be inferred from a weaker one. Empty, skipped,
cancelled, stale or environment-unbound evidence fails closed.
