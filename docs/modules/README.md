---
status: current-candidate
owner: trillionnium-world-architecture
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# Trillionnium World module contracts and detailed designs

This index distinguishes the active game-product workspace from the isolated World-domain authority cutover workspace. A crate is not documentation-complete merely because its name appears in a root README or cross-cutting plan.

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

The active member list is derived from `trillionnium/Cargo.toml`. `trillionnium/crates/platform` is excluded legacy material and is not an active game-product member.

## Isolated World-domain authority cutover workspace

The seven candidate crates under `trillionnium/crates/world-authority/` are not silently added to the active game-product release denominator. Their Cargo-derived index and detailed designs are maintained at [`world-authority/README.md`](world-authority/README.md), and their workspace contract is [`../../trillionnium/crates/world-authority/README.md`](../../trillionnium/crates/world-authority/README.md).

ADR-0003 is binding: these crates may own deterministic World aggregate mutation after a reviewed cutover, while Nakama remains canonical online authority and CEX remains wallet/ledger custody authority. The nested workspace retains `production_authorization=not_granted`.

## Local contract requirements

Every crate README states purpose, authority/non-goals, public/runtime contracts, state/invariants, dependency boundaries, failure/recovery, testing/evidence, compatibility/change control, and a link to the detailed design.

## Detailed design requirements

Every detailed design states, in concrete terms:

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

The documentation gates validate existence, structure, metadata, review expiry, catalogue membership, links, minimum substantive content, traceability, and authority language. They do not prove implementation conformance or release eligibility.

## Cross-cutting precedence

Module documents are subordinate to `PROJECT_BOUNDARY.*`, accepted ADRs, `CURRENT_PLAN.md`, the selected execution snapshot, and normative protocol/database/security/release contracts. A contradiction is a blocker. Module documentation cannot grant public-online, custody, human, legal, commercial, or production authorization credit.

## Update rule

A change that alters public types, owned state, dependency direction, persistence, retry semantics, authority, resource ceilings, compatibility, security, or deployment behavior must update the local contract, detailed design, relevant machine schema/vector, tests, and catalogue in the same pull request.
