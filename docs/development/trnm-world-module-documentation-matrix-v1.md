---
status: current-candidate
owner: trillionnium-world
work_items:
  - WORLD-P1-001
  - WORLD-P1-009
last_reviewed: 2026-09-04
review_due: 2026-09-18
---

# Trillionnium World module documentation matrix v1

The active game-product workspace contains exactly eight crates. Every crate has a local `README.md` that is part of its public review contract. These documents do not replace Rustdoc, schemas, ADRs, runbooks, tests, or external evidence; they provide the stable module entry point that links those layers.

| Crate | Owner | Authority posture | Primary normative neighbors |
|---|---|---|---|
| `trnm-economy-protocol` | World game economy | intents/receipt vocabulary; no custody | settlement ADR, CEX receipt recovery |
| `trnm-rpg-core` | World RPG domain | pure authored-domain vocabulary | closed-loop product contract |
| `trnm-campaign-core` | campaign aggregate | sole persistent RPG mutation | save/battle/economy contracts |
| `trnm-rts-protocol` | RTS command vocabulary | shape/fingerprint only | RTS simulation and online adapter |
| `trnm-rts-sim` | deterministic World battle | unsigned deterministic outcome | transition contract and replay evidence |
| `trnm-online-protocol` | compatibility wire | noncanonical compatibility only | HTTP/WebSocket/error/compatibility docs |
| `trnm-game-server` | compatibility server | `world_legacy_local_alpha` | database, settlement, operations runbooks |
| `trnm-first-contact` | native client | untrusted online client | product flow, packaging, human evidence |

Every module README must contain:

1. purpose;
2. authority and non-goals;
3. public/runtime contracts;
4. state and invariants;
5. dependencies and boundaries;
6. failure and recovery;
7. testing and evidence;
8. compatibility and change control.

`scripts/check-trnm-world-module-documentation.py` derives the member set from `trillionnium/Cargo.toml`, rejects a missing or empty README, rejects missing sections, and rejects authority/release overclaim markers. Its negative self-test proves that deleting a module document or a mandatory section fails the gate.

A README proves documentation presence and reviewability only. It cannot prove compilation, runtime behavior, hosted CI, server-side governance, cross-repository compatibility, deployment, custody, human usability, legal approval, or production authorization.
