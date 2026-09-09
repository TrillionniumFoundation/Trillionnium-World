---
status: current-candidate
owner: trillionnium-world-campaign
module: trnm-campaign-core
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-campaign-core detailed design

## Scope and authority

`trnm-campaign-core` is the sole game-domain aggregate allowed to mutate persistent RPG campaign state. It creates versioned battle seeds, validates and stages battle results, advances quests/chapters/progression, manages inventory/equipment/resources, records replay/value references, maintains save slots/settings, and projects independently verified economic receipts. It does not render UI, run Bevy systems, admit/order online players, sign completion evidence, call CEX, or own wallet state.

## Public interfaces

The public command/query surface is organized around `CampaignSaveV1`, battle seed/result contracts, settlement receipts, quest/encounter/relationship/inventory/equipment/crafting commands, save-slot operations, player settings, and migration/validation helpers. Stable identities include `trnm_campaign_save_v1`, `trnm_battle_seed_v8`, `trnm_battle_result_v2`, and `trnm_settlement_receipt_v1`; the current save schema revision is 12. Commands return a typed success delta/view or `CampaignError` with no hidden partial result.

## State model and invariants

The principal phases are `Town`, `BattlePending`, and `PostBattlePending`. A public command executes against a cloned candidate aggregate, validates every affected invariant, and replaces the published aggregate only after complete success. Every `Err` therefore preserves the full serialized state. Invariants include valid phase transitions; exact four-member active party; unique battle/economic/idempotency identities; nonnegative bounded values; seed/result/rules/content/hash agreement; one-to-one settlement links; valid quest/story graphs; and no wallet value inferred from local soft credits.

## Dependency direction

The aggregate depends on `trnm-rpg-core` and `trnm-economy-protocol`, and exposes battle types consumed by RTS simulation. It is Bevy-free and must not import network clients, service credentials, SQL adapters, process-global environment policy, or remote clocks. File save helpers are an adapter kept behind the aggregate invariant boundary; network/server/client workers submit typed outcomes through public commands. Domain modules must not depend back on presentation or transport crates.

## Execution, concurrency and cancellation

One mutable aggregate revision is admitted at a time. Commands are deterministic over the current state and explicit input; callers performing asynchronous work capture the expected campaign revision/state hash and revalidate before apply. No remote I/O occurs while an aggregate mutation is in progress. Cancellation before publication discards the candidate. Concurrent server writers use database row/CAS/fence rules; local saves use process/file locking where supported. Lock order is owned by the server/database contract, not reinvented inside pure domain code.

## Persistence and durability

Save slots and settings use bounded, versioned, hash/shape-checked data and atomic replacement. Durable battle settlement is staged before final apply so restart can converge one pending result exactly once. Connected economy uses a durable intent outbox, priority compensation lane, verified receipts, reconciliation cursor, and tombstones. Filesystem success requires write, flush/fsync as specified, atomic rename, directory durability where required, ownership/mode checks, and corruption isolation. Unsupported or corrupt saves fail closed rather than being partially interpreted.

## Idempotency, retry and failure taxonomy

Battle, economic, and settlement identities are immutable. An exact duplicate battle result or receipt returns a zero-delta/idempotent outcome after verifying every binding; altered bytes under the same identity fail. Queue full, reserve failure, budget exhaustion, receipt mismatch, stale expected revision, invalid phase, unsupported schema/version, integrity failure, and I/O failure are distinct families. A recoverable transport failure leaves an intent pending; it never commits local wallet success. Restart recovery resumes staged work from durable evidence without duplicating progression or value.

## Versioning and migration

Persisted-field semantic changes require a new save schema revision and deterministic readers/migrations from every supported predecessor. Battle seed/result, settlement, replay reference, or economy projection changes require independent contract versions, positive/negative vectors, client/simulation/server compatibility tests, rollback notes, and a reader-retirement inventory. Migrations are monotonic and state preserving: a failed migration leaves the original bytes recoverable. Content/rules revision changes are explicit and cannot be inferred from build labels.

## Resource and performance budgets

Save size, party/inventory/quest/event/outbox/dead-letter/tombstone counts, replay references, dialogue/memory records, daily issuance, and command batch sizes must be bounded. Mutation cost should be proportional to affected bounded collections rather than total historical state; unbounded scans require indexes or compaction policy. Atomic candidate cloning has a measurable cost and must be benchmarked as state grows. Any budget change updates validation, fixtures, migration behavior, UI status, and release capacity evidence.

## Observability and security

Domain errors expose stable codes and bounded context, never credentials or unbounded remote bodies. Evidence records command type, aggregate identity/revision, before/after state hashes on success, unchanged hash on error, battle/intent/receipt identities, and migration result. Local files use safe paths, no symlink traversal, restrictive modes for journals, and corruption isolation. Client-provided results/receipts are untrusted until verified against authoritative seed, participant, account, policy, hash, and signature material.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-campaign-core/src/` and its ownership manifest. Normative neighbors include `GAME_STATUS.md`, `docs/development/trillionnium-rpg-rts-closed-loop-v1.md`, settlement lifecycle/receipt documents, and `docs/development/trnm-world-testing-strategy-v2.md`. Tests cover every supported migration, command error-state byte preservation, queue/reserve/budget failures, duplicate/altered battle/receipt, mid-settlement restart, save crash consistency, quest retries, two consecutive battles with reload, and campaign/RTS/CEX integration.

## Change checklist and open work

Before changing a command: state pre/postconditions, affected fields, invariants and stable errors; prove candidate-only mutation and error-state hash equality; update schema/version/migration when required; add duplicate/stale/overflow/fault/restart cases; update client/server adapters and documentation; and measure state-size impact. Open work includes converting ownership `include!` parts into true Rust modules, generating a complete command/state-transition catalogue, publishing field-level save schemas/migration matrix, and obtaining exact-head hosted/fault evidence.
