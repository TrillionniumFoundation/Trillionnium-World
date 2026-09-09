---
status: current-candidate
owner: trillionnium-world-rpg
module: trnm-rpg-core
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-rpg-core detailed design

## Scope and authority

`trnm-rpg-core` owns deterministic RPG vocabulary and authored-content definitions: character attributes/origins/builds, items and equipment effects, skills and techniques, encounters, NPC identities/schedules/relationships, quests and condition graphs, rooms/routes, markets, crafting, factions, and original narration. It is a pure domain/content library. It does not persist campaign state, perform settlement, admit online players, order network commands, render UI, or own wallet/finality authority.

## Public interfaces

The public surface consists of typed IDs/enums/records, catalogue constants, world-graph and route helpers, quest-condition and narrative accessors, market pricing/content helpers, NPC schedule/dialogue/social helpers, skill/equipment validation, and deterministic encounter-resolution primitives. Stable identifiers are explicit lowercase data keys; display/localized strings are presentation only and must never be parsed to reconstruct identity, permission, or economic meaning. Consumers receive typed values or bounded errors, not implicit string conventions.

## State model and invariants

The crate persists no mutable state. Its authored catalogues and pure helpers must preserve uniqueness of every room, edge, NPC, quest, item, skill, recipe, faction, encounter, and content revision; valid graph endpoints and story locks; reachable quest DAGs with explicit terminal outcomes; exhaustive typed effects for non-material equipment; bounded attributes, relationships, prices, schedules, and encounter values; and original/provenance-reviewed content. Reusing or silently changing a stable ID is a breaking migration.

## Dependency direction

`trnm-rpg-core` sits below `trnm-campaign-core` and presentation/runtime adapters. It may use deterministic collections, serialization, and hashing, but must not import Bevy, SQL, HTTP, filesystem state, environment variables, wall-clock time, nondeterministic randomness, campaign persistence, or service credentials. Content import/build tools may validate external source files, but generated current product data must be reproducible, reviewed, and checked into or bound to an immutable content artifact.

## Execution, concurrency and cancellation

All domain helpers are synchronous and deterministic for identical typed inputs and content revision. There are no tasks, locks, remote calls, or cancellation points. Callers may evaluate content concurrently because catalogue values are immutable; any cache must preserve exact revision identity and cannot change semantics. Encounter/quest helpers must compute a candidate result before the campaign aggregate publishes it. Nondeterminism, including authored random choices, enters only through an explicit versioned seed owned by the caller.

## Persistence and durability

Persistence belongs to `trnm-campaign-core` and the client/server storage adapters. Saves store stable IDs and revisioned state, never pointers, display labels, or process-local catalogue indices. A reader must reject unknown/retired IDs unless a declared migration maps them. Content packages bind catalogue bytes, map/asset identities, provenance, license inventory, and a content digest. Missing content cannot be silently substituted with another item, quest, NPC, route, or encounter because that would alter deterministic behavior.

## Idempotency, retry and failure taxonomy

Pure queries are naturally repeatable. Domain transitions return typed rejection for unknown IDs, invalid graph edges, unmet locks/prerequisites, invalid quest transitions, malformed content, out-of-range values, and crossed content/rules revisions. A rejection has no side effect. The campaign owner decides whether a player command can be retried, but it must not reinterpret an invalid identifier or silently choose a fallback. Stable errors should separate content-not-found, invalid-state, prerequisite, bounds, integrity, and unsupported-revision families.

## Versioning and migration

Content revision and gameplay/ruleset version are explicit and distinct from binary build identity. Adding content may be backward compatible only when old saves/readers have deterministic behavior and no identifier meaning changes. Renaming/reusing an ID, changing graph connectivity, formulas, item effects, quest semantics, market interpretation, or encounter ordering requires a rules/content revision, save migration/reader policy, updated hashes/vectors, rollback notes, and campaign/client compatibility tests. Retirement needs a usage inventory and date.

## Resource and performance budgets

Catalogue sizes, identifier/text lengths, graph vertices/edges, quest DAG depth/branching, route length, recipe inputs, market ranges, dialogue options, and encounter action counts must be explicitly bounded by validators. Graph and lookup operations should use deterministic indexed maps/sets and have documented worst-case complexity. New content must pass startup/load-time and memory budgets on supported client/server profiles. A large content addition requires measured parse/load/render impact rather than a narrative performance claim.

## Observability and security

Content validation emits stable IDs, revision/digest, and bounded diagnostics without leaking player or credential data. Release artifacts retain source/provenance/license information for text, art, audio, maps, and references. The clean-room boundary is mandatory: no proprietary source code, maps, tables, text, art, music, or private data may be copied. Modded or untrusted content is not admitted as current authority without signature/revision policy, strict schema limits, path safety, and sandbox/resource review.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-rpg-core/src/`. Product truth is summarized in `GAME_STATUS.md` and `docs/development/trillionnium-rpg-rts-closed-loop-v1.md`. Required tests cover catalogue/ID uniqueness, graph reachability and locks, quest DAG validity, item/equipment exhaustiveness, formula/market bounds, NPC schedule determinism, encounter preservation, stable serialization/content hashes, migration fixtures, and provenance inventory. Client screenshots or play sessions are higher evidence classes and cannot be inferred from unit tests.

## Change checklist and open work

For every change: declare the affected IDs and revision; update content schema/catalogue, migration behavior, deterministic fixtures and hashes; verify graph/DAG/effect exhaustiveness; record provenance/license/localization; run campaign/client regressions; and review resource impact. Open work includes machine-readable schemas for rooms/NPCs/quests/items/skills/recipes, generated world/quest visualizations, explicit formula reference, localization keys, content-authoring/validation tooling, and a dated stable-ID deprecation process.
