# trnm-campaign-core

Status: current candidate  
Owner: Trillionnium World campaign/save/progression aggregate

## Purpose

This crate is the sole game-domain authority for persistent RPG campaign mutation. It creates battle seeds, validates battle results, advances quests and progression, manages inventory and equipment, records replay and value-event references, and performs idempotent local projection of verified economic receipts.

## Authority and non-goals

Presentation clients may request a `BattleSeedV1` and submit a `BattleResultV1`, but only this aggregate may mutate campaign progression. It does not render UI, run Bevy systems, admit online participants, assign canonical online order, sign completion evidence, or own CEX wallet state.

## Public contracts

Stable contract names include `trnm_campaign_save_v1`, `trnm_battle_seed_v8`, `trnm_battle_result_v2`, and `trnm_settlement_receipt_v1`. The current save schema revision is 12. Public methods are commands over `CampaignSaveV1`; storage helpers provide atomic file replacement and migration of supported older revisions.

## State and invariants

Campaign phases are `Town`, `BattlePending`, and `PostBattlePending`. Every public command executes against a cloned candidate and publishes it only after command success and aggregate validation. Therefore every `Err` is byte/state preserving. Additional invariants include unique battle and economic identities, exact four-member active party, non-negative bounded balances, one-to-one settlement links, valid quest/story graphs, and exact seed/result/hash agreement.

## Dependencies and boundaries

The crate is Bevy-free and depends on `trnm-rpg-core` and `trnm-economy-protocol`. It may serialize and atomically persist campaign files but must not call remote services. Network adapters and asynchronous workers live in client/server layers and submit typed outcomes back through public APIs.

## Failure and recovery

A battle result is first staged durably, then applied once. Restart recovery completes a pending settlement; an exact duplicate returns zero delta, while changed bytes for the same identity fail closed. Economy requests remain pending until a verified receipt is applied. Corrupt or unsupported saves are rejected rather than partially interpreted.

## Testing and evidence

Required coverage includes migration from every supported schema, command error-state preservation, save crash consistency, duplicate and altered battle results, mid-settlement restart, queue and budget limits, receipt mismatch, idempotency tombstones, quest retries, and two consecutive battles with reload. Exact-head tests are required for verification.

## Compatibility and change control

A persisted-field semantic change requires a schema revision and migration. Battle seed/result or settlement payload changes require a contract version, vectors, client/simulation compatibility tests, and rollback notes. Removing a reader is forbidden until the supported-save inventory and retirement date are recorded.
