---
status: current-candidate
owner: trillionnium-world-architecture
work_items:
  - WORLD-P1-001
  - WORLD-P1-009
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Trillionnium World module documentation matrix v1

The active game-product workspace contains exactly eight crates. Every crate has a local `README.md` review contract and one detailed design registered in `docs/catalog.json`. These documents do not replace Rustdoc, schemas, ADRs, runbooks, tests, hosted execution, or external evidence.

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

The local contract gate derives member identities from `trillionnium/Cargo.toml`, rejects missing or shallow README sections and authority overclaims, and requires exact matrix membership. The detailed-design gate additionally checks the twelve design sections, review metadata/expiry, expected module-specific terms, traceability, catalogue membership, and discoverable links.

A documentation PASS means that a stable review surface exists. It cannot prove compilation, state-machine correctness, database behavior, hosted CI, server-side governance, cross-repository compatibility, deployment, custody, human usability, legal approval, or production authorization.
