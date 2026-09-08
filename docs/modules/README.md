---
status: current-candidate
owner: trillionnium-world
last_reviewed: 2026-09-08
review_due: 2026-10-08
---

# Trillionnium World module contracts

The active game-product workspace contains eight Rust crates. Each crate has a concise local README and a separate detailed technical design. Documentation presence is checked mechanically, but implementation conformance, hosted CI, deployment and release evidence remain independent denominators.

| Module | Authority summary | Technical design |
|---|---|---|
| `trnm-economy-protocol` | Defines game-owned immutable economic intents and receipt vocabulary. | [`trnm-economy-protocol.md`](trnm-economy-protocol.md) |
| `trnm-rpg-core` | Owns deterministic RPG vocabulary and authored content definitions. | [`trnm-rpg-core.md`](trnm-rpg-core.md) |
| `trnm-campaign-core` | Sole game-domain authority for persistent RPG progression. | [`trnm-campaign-core.md`](trnm-campaign-core.md) |
| `trnm-rts-protocol` | Defines command shape and strict intake only. | [`trnm-rts-protocol.md`](trnm-rts-protocol.md) |
| `trnm-rts-sim` | Owns deterministic World battle behavior and unsigned game-domain outcome material. | [`trnm-rts-sim.md`](trnm-rts-sim.md) |
| `trnm-online-protocol` | Compatibility message vocabulary only. | [`trnm-online-protocol.md`](trnm-online-protocol.md) |
| `trnm-game-server` | Bounded `world_legacy_local_alpha` compatibility enclave. | [`trnm-game-server.md`](trnm-game-server.md) |
| `trnm-first-contact` | Owns presentation, input and local orchestration. | [`trnm-first-contact.md`](trnm-first-contact.md) |

## Contract requirements

Every design must describe current implementation; responsibilities and non-responsibilities; public interfaces; state and invariants; dependency direction; concurrency/cancellation; durability/recovery; failure/retry/idempotency; versioning/migration; resource budgets; observability/security; verification traceability; and known gaps/change checklist.

The machine inventory is [`contracts-v2.json`](contracts-v2.json). `scripts/check-trnm-world-module-documentation.py` derives the active packages from `trillionnium/Cargo.toml`, requires an exact one-to-one inventory, verifies local links and rejects stale source-generation assertions, personal paths and authority/release overclaims.

## Change rule

A pull request that changes a public type, owned state, persistence, lock order, cancellation, retry identity, resource ceiling, compatibility or deployment behavior must update the relevant README, detailed design, schema/vector and tests in the same review stack. Documentation alone never grants production authorization.
