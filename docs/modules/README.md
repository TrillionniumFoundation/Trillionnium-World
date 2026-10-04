---
status: current-candidate
owner: trillionnium-world-architecture
last_reviewed: 2026-09-09
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Trillionnium World module contracts and detailed designs

This index distinguishes active game-product modules, the isolated World-domain authority cutover candidate, scope-dependent external-contract MVPs and retained historical source. A crate is not documentation-complete merely because its name appears in a root README or cross-cutting plan. `../component-catalog.json` is the complete physical manifest/lifecycle inventory.

## Active game-product workspace

| Module | Authority/responsibility | Local contract | Detailed design |
|---|---|---|---|
| `trnm-economy-protocol` | game-owned intent/receipt vocabulary; no wallet custody | [`../../trillionnium/crates/trnm-economy-protocol/README.md`](../../trillionnium/crates/trnm-economy-protocol/README.md) | [`trnm-economy-protocol-design.md`](trnm-economy-protocol-design.md) |
| `trnm-rpg-core` | deterministic RPG vocabulary, content rules, and world graph | [`../../trillionnium/crates/trnm-rpg-core/README.md`](../../trillionnium/crates/trnm-rpg-core/README.md) | [`trnm-rpg-core-design.md`](trnm-rpg-core-design.md) |
| `trnm-campaign-core` | sole persistent RPG campaign/save/progression mutation | [`../../trillionnium/crates/trnm-campaign-core/README.md`](../../trillionnium/crates/trnm-campaign-core/README.md) | [`trnm-campaign-core-design.md`](trnm-campaign-core-design.md) |
| `trnm-rts-protocol` | deterministic RTS command and strict-intake vocabulary | [`../../trillionnium/crates/trnm-rts-protocol/README.md`](../../trillionnium/crates/trnm-rts-protocol/README.md) | [`trnm-rts-protocol-design.md`](trnm-rts-protocol-design.md) |
| `trnm-rts-sim` | Bevy-free deterministic RTS simulation and replay material | [`../../trillionnium/crates/trnm-rts-sim/README.md`](../../trillionnium/crates/trnm-rts-sim/README.md) | [`trnm-rts-sim-design.md`](trnm-rts-sim-design.md) |
| `trnm-online-protocol` | World-local compatibility wire vocabulary, never canonical online authority | [`../../trillionnium/crates/trnm-online-protocol/README.md`](../../trillionnium/crates/trnm-online-protocol/README.md) | [`trnm-online-protocol-design.md`](trnm-online-protocol-design.md) |
| `trnm-game-server` | bounded `world_legacy_local_alpha` compatibility enclave and settlement runtime | [`../../trillionnium/crates/trnm-game-server/README.md`](../../trillionnium/crates/trnm-game-server/README.md) | [`trnm-game-server-design.md`](trnm-game-server-design.md) |
| `trnm-first-contact` | native Bevy presentation, input, local orchestration, and compatibility transport | [`../../trillionnium/crates/trnm-first-contact/README.md`](../../trillionnium/crates/trnm-first-contact/README.md) | [`trnm-first-contact-design.md`](trnm-first-contact-design.md) |

The active member list is derived from `trillionnium/Cargo.toml`. It is the only active game-product Cargo denominator.

## Isolated World-domain authority cutover workspace

The seven candidate crates under `trillionnium/crates/world-authority/` are not silently added to the active game-product release denominator. Their Cargo-derived index and detailed designs are maintained at [`world-authority/README.md`](world-authority/README.md), and their workspace contract is [`../../trillionnium/crates/world-authority/README.md`](../../trillionnium/crates/world-authority/README.md).

ADR-0003 is binding: these crates may own deterministic World aggregate mutation after a reviewed cutover, while Nakama remains canonical online authority and CEX remains wallet/ledger custody authority. The nested workspace retains `production_authorization=not_granted`.

## Scope-dependent external-contract MVP workspace

The four crates under `contracts/` are catalogued and documented at [`contracts/README.md`](contracts/README.md). They freeze Rust MVP semantics and shared audit vocabulary only:

| Module | Current responsibility | Detailed design |
|---|---|---|
| `audit-events` | normalized audit-event values | [`contracts/audit-events-design.md`](contracts/audit-events-design.md) |
| `settlement-vault` | in-memory settlement-vault state machine | [`contracts/settlement-vault-design.md`](contracts/settlement-vault-design.md) |
| `bridge-relay` | in-memory proof/nonce/finality relay state machine | [`contracts/bridge-relay-design.md`](contracts/bridge-relay-design.md) |
| `governance-guard` | in-memory timelock/pause/version state machine | [`contracts/governance-guard-design.md`](contracts/governance-guard-design.md) |

They are not connected to a canonical host ABI/runtime, deterministic WASM sandbox, durable state root, production RPC, custody or public-mainnet authorization. Day-1 inclusion remains scope-dependent.

## Retained historical source

- `trillionnium/crates/platform/`: 12-crate excluded legacy workspace, documented by its own README and classified with `release_denominator=none`.
- `web4-frontend/`: historical-compatible Node application outside the World game-product denominator, with its own developer/API/test/operations documentation.

These components need an explicit lifecycle and provenance surface, but their existence cannot inflate current product completion. New semantic development in the legacy platform is prohibited without a new ADR and accountable-repository decision.

## Local contract requirements

Every active or candidate crate README states purpose, authority/non-goals, public/runtime contracts, state/invariants, dependency boundaries, failure/recovery, testing/evidence, compatibility/change control, and a link to detailed design. Scope-dependent MVP crates must state equally explicit non-production and host-runtime boundaries. Excluded legacy members may share one workspace freeze contract only because semantic development is prohibited.

## Detailed design requirements

Every active, candidate or maintained MVP detailed design states, in concrete terms:

1. scope and authority;
2. public interfaces;
3. state model and correctness invariants;
4. dependency direction and forbidden adapters;
5. execution, concurrency, cancellation, and lock-order posture;
6. persistence and durable/private/public boundaries;
7. idempotency, retry, and stable failure taxonomy;
8. versioning, compatibility, and migration rules;
9. resource/performance budgets and measurement ownership;
10. observability and security boundaries;
11. source/test/protocol/runbook evidence traceability;
12. change checklist and honestly open work.

The documentation gates validate existence, structure, metadata, review expiry, catalogue membership, links, minimum substantive content, traceability and authority language. The component-catalog gate additionally rejects any physical Cargo/Node manifest without an explicit lifecycle and owner. These gates do not prove implementation conformance or release eligibility.

## Cross-cutting precedence

Module documents are subordinate to `PROJECT_BOUNDARY.*`, accepted ADRs, `CURRENT_PLAN.md`, the selected execution snapshot and normative protocol/database/security/release contracts. A contradiction is a blocker. Module documentation cannot grant public-online, custody, human, legal, commercial or production authorization credit.

## Update rule

A change that alters public types, owned state, dependency direction, persistence, retry semantics, authority, resource ceilings, compatibility, security or deployment behavior must update the local contract, detailed design, relevant machine schema/vector, tests and both applicable catalogues in the same pull request.
