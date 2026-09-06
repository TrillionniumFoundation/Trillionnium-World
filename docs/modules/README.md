---
status: current-candidate
owner: trillionnium-world
last_reviewed: 2026-09-04
review_due: 2026-10-04
---

# Trillionnium World module contracts

This index maps each active Rust workspace member to its module-level technical contract. A crate is not considered documentation-complete merely because its name appears in the root README or a cross-cutting plan.

## Active game-product workspace

| Module | Authority/responsibility | Module contract |
|---|---|---|
| `trnm-economy-protocol` | game-owned economic intent/receipt vocabulary; no wallet custody | [`../../trillionnium/crates/trnm-economy-protocol/README.md`](../../trillionnium/crates/trnm-economy-protocol/README.md) |
| `trnm-rpg-core` | RPG domain vocabulary, content rules and world graph | [`../../trillionnium/crates/trnm-rpg-core/README.md`](../../trillionnium/crates/trnm-rpg-core/README.md) |
| `trnm-campaign-core` | campaign/save/progression aggregate and local settlement | [`../../trillionnium/crates/trnm-campaign-core/README.md`](../../trillionnium/crates/trnm-campaign-core/README.md) |
| `trnm-rts-protocol` | deterministic RTS command vocabulary | [`../../trillionnium/crates/trnm-rts-protocol/README.md`](../../trillionnium/crates/trnm-rts-protocol/README.md) |
| `trnm-rts-sim` | Bevy-free deterministic RTS simulation | [`../../trillionnium/crates/trnm-rts-sim/README.md`](../../trillionnium/crates/trnm-rts-sim/README.md) |
| `trnm-online-protocol` | World-local online compatibility vocabulary | [`../../trillionnium/crates/trnm-online-protocol/README.md`](../../trillionnium/crates/trnm-online-protocol/README.md) |
| `trnm-game-server` | bounded `world_legacy_local_alpha` compatibility enclave | [`../../trillionnium/crates/trnm-game-server/README.md`](../../trillionnium/crates/trnm-game-server/README.md) |
| `trnm-first-contact` | native Bevy presentation, input and local client orchestration | [`../../trillionnium/crates/trnm-first-contact/README.md`](../../trillionnium/crates/trnm-first-contact/README.md) |

The active member list is defined by `trillionnium/Cargo.toml`. `trillionnium/crates/platform` is excluded legacy material and is not an active game-product member.

## Mandatory contract sections

Every active crate contract must state, in concrete terms:

1. responsibilities and explicit non-responsibilities;
2. authority and data ownership;
3. dependency direction and forbidden adapters;
4. owned state and correctness invariants;
5. concurrency, cancellation and lock-order posture where applicable;
6. durable/private/public boundaries;
7. idempotency, retry and failure behavior;
8. versioning, compatibility and migration rules;
9. resource and performance budgets;
10. observability and security boundaries;
11. local and external evidence requirements;
12. change checklist and known open work.

The repository documentation checker enforces existence and structural coverage. It does not prove implementation conformance; source/tests/evidence remain separate denominators.

## Cross-cutting specifications

Module contracts are subordinate to:

- `PROJECT_BOUNDARY.md` and `PROJECT_BOUNDARY.json`;
- accepted ADRs;
- `CURRENT_PLAN.md` and its authoritative execution snapshot;
- normative protocol, database, security and release contracts.

A contradiction is a blocker. Module documentation cannot grant public-online, custody, human, legal, commercial or production authorization credit.

## Update rule

A change that alters a module's public types, owned state, dependency direction, persistence, retry semantics, authority, resource ceiling, compatibility or deployment behavior must update the corresponding module contract in the same pull request. Documentation-only assertions do not replace executable tests or exact-head evidence.