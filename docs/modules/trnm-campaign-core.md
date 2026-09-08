---
status: current-candidate
owner: Trillionnium World campaign/save/progression aggregate
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-campaign-core technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A Bevy-free aggregate that creates battle seeds, validates battle results, advances quests and progression, manages inventory/equipment, records replay and value-event references, performs idempotent projection of verified economic receipts, and atomically stores supported save revisions.

## Responsibilities and non-responsibilities

Sole game-domain authority for persistent RPG progression. Presentation, online admission/order, completion signing, Chain finality and CEX wallet state are outside this crate.

## Public interfaces and data contracts

Stable identities include `trnm_campaign_save_v1`, `trnm_battle_seed_v8`, `trnm_battle_result_v2`, and `trnm_settlement_receipt_v1`. Commands operate on `CampaignSaveV1`; storage helpers load, validate, migrate and atomically replace save files. Public commands must document preconditions, state delta and stable error category.

## State model and invariants

Phases are `Town`, `BattlePending` and `PostBattlePending`. Every command executes on a cloned candidate and publishes only after validation, so an error preserves the exact prior aggregate bytes. Invariants include unique battle/economic identities, valid party membership, bounded balances, one-to-one settlement links, exact seed/result hashes and valid quest/story transitions.

## Dependency direction

Depends on `trnm-rpg-core` and `trnm-economy-protocol`; it may serialize and atomically persist campaign files. It must not call network services or import Bevy/server adapters. Transport workers submit typed outcomes through public aggregate APIs. Storage adapters must not bypass aggregate validation.

## Concurrency and cancellation

The aggregate is single-writer per save/campaign identity. Callers serialize mutations or use optimistic revision/hash compare-and-swap. File helpers use process-level ownership and atomic replace; they do not make concurrent multi-process mutation safe. Cancellation before publication leaves the original state visible.

## Durability and recovery

Save writes use a temporary file in the same directory, bounded serialization, flush/sync as specified by the platform adapter, atomic rename and parent-directory durability where supported. Battle results are staged before apply. Pending economic intents and receipts survive restart. Corrupt or unsupported saves are isolated rather than partially interpreted.

## Failure, retry and idempotency

Invalid commands, altered duplicate results, receipt mismatch, unsupported revision, corrupt storage and impossible transitions fail closed. Exact duplicates return an idempotent no-op where defined. Response loss remains pending until a verified receipt is applied. Recovery resumes staged battle/economy work from durable identity and never reissues value under a new ID.

## Versioning and migration

Persisted semantic changes require a save schema revision plus migrations from every supported prior revision. Battle seed/result or receipt changes require new contract versions and client/simulation vectors. Reader retirement requires usage inventory, retention window, migration rehearsal and rollback artifacts.

## Resource budgets and performance

Save bytes, slots, inventory rows, quest/event history, pending intents, dead letters and replay references are bounded. Commands must be linear or better in the bounded aggregate size. No command may allocate from untrusted declared counts without validation. Budget values are centralized and included in migration tests.

## Observability and security

The aggregate returns structured outcomes; adapters log stable campaign ID digest, command type, revision transition and error code. Do not log full player PII, credentials or signed receipts. Security review covers forged results, replayed receipts, path/symlink handling, save tampering and local-to-wallet escalation.

## Verification traceability

Required tests cover all migrations, error-state byte preservation, crash-consistent writes, duplicate/altered battle results, mid-settlement restart, queue/budget limits, receipt mismatch, tombstones, quest retries and consecutive battles across reload. Property tests compare before/after canonical hashes on every rejected command.

## Known gaps and change checklist

Publish the complete command/state transition table and field-level save schema; isolate pure aggregate from file storage behind a trait; add platform-specific durability evidence; and bind every public command to test IDs. Any mutation path, migration or budget change updates the design and fixtures together.
