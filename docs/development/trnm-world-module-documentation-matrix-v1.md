---
status: current-candidate
owner: trillionnium-world-architecture
work_items:
  - WORLD-P1-001
  - WORLD-P1-009
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Trillionnium World module documentation matrix v1

Documentation coverage has two different denominators and they must not be conflated:

1. **active/candidate semantic modules** require a local contract plus a substantive detailed design;
2. **every physical source/application manifest** requires an explicit lifecycle, owner, release denominator, canonical documentation and gate in `docs/component-catalog.json`.

A file or directory is not current merely because it remains in the repository, and a documentation PASS cannot prove implementation conformance or release eligibility.

## Active game-product modules: 8/8 design surfaces

The active game-product workspace contains exactly eight crates. Every crate has a local `README.md` review contract and one detailed design under `docs/modules/`.

| Crate | Owner | Authority posture | Detailed design | Primary normative neighbors |
|---|---|---|---|---|
| `trnm-economy-protocol` | World game economy | intent/receipt vocabulary; no custody | `docs/modules/trnm-economy-protocol-design.md` | settlement ADR, receipt recovery, CEX compatibility |
| `trnm-rpg-core` | World RPG domain | pure authored-domain vocabulary | `docs/modules/trnm-rpg-core-design.md` | closed-loop product contract, content provenance |
| `trnm-campaign-core` | campaign aggregate | sole persistent RPG mutation | `docs/modules/trnm-campaign-core-design.md` | save/battle/economy contracts |
| `trnm-rts-protocol` | RTS command vocabulary | shape/fingerprint/strict intake only | `docs/modules/trnm-rts-protocol-design.md` | intake schema/vectors and runtime adapters |
| `trnm-rts-sim` | deterministic World battle | unsigned deterministic result/replay | `docs/modules/trnm-rts-sim-design.md` | transition contract and replay evidence |
| `trnm-online-protocol` | compatibility wire | noncanonical compatibility only | `docs/modules/trnm-online-protocol-design.md` | HTTP/WebSocket/error/compatibility contracts |
| `trnm-game-server` | compatibility server | `world_legacy_local_alpha` | `docs/modules/trnm-game-server-design.md` | database, settlement, identity, operations runbooks |
| `trnm-first-contact` | native client | untrusted online client | `docs/modules/trnm-first-contact-design.md` | product flow, assets, packaging, human evidence |

The local contract gate derives member identities from `trillionnium/Cargo.toml`, rejects missing or shallow README sections and authority overclaims, and requires exact matrix membership. The detailed-design gate checks the twelve design sections, review metadata/expiry, module-specific terms, traceability, catalogue membership and discoverable links.

## Isolated World-domain authority candidate: 7/7 design surfaces

The separate `trillionnium/crates/world-authority/Cargo.toml` workspace contains:

- `trnm-world-domain`;
- `trnm-world-command`;
- `trnm-world-projection`;
- `trnm-world-map-provider`;
- `trnm-world-ui-fragments`;
- `trnm-world-api`;
- `trnm-world-server`.

Each has a local crate contract and detailed design under `docs/modules/world-authority/`. These documents describe a cutover candidate, not a production service. Fixture adapters, a file repository and loopback-only server remain non-production; `production_authorization=not_granted` is binding.

## Scope-dependent external-contract MVPs: 4/4 design surfaces

The independent `contracts/` workspace contains:

- `audit-events`;
- `settlement-vault`;
- `bridge-relay`;
- `governance-guard`.

Each has a detailed design under `docs/modules/contracts/`, validated by `scripts/check-trnm-world-contract-module-documentation.py`. They remain Rust in-memory/shared-schema MVPs. The canonical host ABI, runtime specification, deterministic WASM sandbox, gas/storage model, durable state, state-root/RPC integration and production authorization are absent.

## Full physical repository inventory

`docs/component-catalog.json` additionally classifies source that must not be counted as active product completion:

| Physical component class | Count | Lifecycle / release denominator |
|---|---:|---|
| Active game-product crates | 8 | active or compatibility / `game-product` |
| World-domain authority crates | 7 | cutover/fixture candidate / `world-authority-candidate` |
| Legacy platform crates | 12 | `excluded-legacy` / `none` |
| External-contract MVP crates | 4 | `mvp-perimeter` / `scope-dependent` |
| Web4 application | 1 | `historical-compatible-subproject` / `none` |

Workspace manifests are also separately catalogued. The component-catalog gate scans every `Cargo.toml` under `trillionnium/` and `contracts/`, plus `web4-frontend/package.json`; an unclassified or stale manifest is a failure.

## What the gates establish

A module/documentation PASS establishes only that:

- every active/candidate semantic crate has an accountable review surface;
- all physical Rust/Node manifests have an explicit lifecycle and release denominator;
- required sections, links, ownership, review dates and non-overclaim language exist;
- known stale source/document contradictions are rejected.

It cannot establish:

- compilation or state-machine correctness;
- database/journal/settlement behavior;
- hosted exact-head or prospective-merge execution;
- server-side branch protection or eligible review;
- cross-repository protocol compatibility;
- deployment, public edge, cross-host recovery or endurance;
- custody/KMS, human/accessibility, privacy, legal or commercial approval;
- production authorization.

## Required commands

```bash
python3 scripts/check-trnm-world-component-catalog.py
python3 scripts/test-trnm-world-component-catalog.py
python3 scripts/check-trnm-world-current-conformance.py
python3 scripts/test-trnm-world-current-conformance.py
python3 scripts/check-trnm-world-module-documentation.py
python3 scripts/test-trnm-world-module-documentation-negative.py
python3 scripts/check-trnm-world-detailed-documentation.py --as-of "$(date -u +%F)"
python3 scripts/test-trnm-world-detailed-documentation.py
python3 scripts/check-trnm-world-authority-documentation.py --as-of "$(date -u +%F)"
python3 scripts/test-trnm-world-authority-documentation.py
python3 scripts/check-trnm-world-contract-module-documentation.py --as-of "$(date -u +%F)"
python3 scripts/test-trnm-world-contract-module-documentation.py
```

A lower evidence class cannot satisfy a higher one by implication. Missing, skipped, cancelled, stale or identity-unbound execution is a blocker, not a pass.
