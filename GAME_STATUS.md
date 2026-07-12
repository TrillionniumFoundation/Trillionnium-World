# TRNM Game Status

Updated: 2026-07-12

This is the one-page status source for the current native RPG + real-time-strategy product. The older finite checklist in `docs/development/trnm-deep-rpg-complete-rts-v1-dod.md` is retained as a completed historical baseline, not as a claim that the broader deep-RPG + complete-RTS vision is 100%. The current local CEX economy integration is enumerated below; it does not grant blockchain, Android, multiplayer or public-launch credit.

Release denominators are separated by
`docs/development/trnm-native-game-release-gates-v1.md`: software alpha,
commercial single-player, trusted CEX settlement and public player market are
four different gates.

The bounded v2 checklist remains historical evidence; this page records the newer runtime state directly and does not create another artificial "100%" contract.

## Product boundary

- Product workspace: six crates in `trillionnium/Cargo.toml`.
- Stable game-owned economy boundary: `trnm-economy-protocol`.
- Native client: `trnm-first-contact`.
- RPG/world vocabulary: `trnm-rpg-core`.
- Campaign, save, progression and settlement authority: `trnm-campaign-core`.
- Player-order contract: `trnm-rts-protocol`.
- Bevy-free deterministic battle authority: `trnm-rts-sim`.
- Historical legacy game workspace: absent; see `docs/archive/frozen-legacy-final-index-2026-07-11.md`.
- CEX settlement backend: optional HTTP runtime behind the game-owned protocol; the native client never uses the CEX Web/Matrix game shell.

## Current authored scope

### RPG

- twenty connected rooms across Mirror City, Signal Road, Glass Basin and the Ashen Fringe, with story locks and authoritative route planning; Glass Basin and Ashen Fringe each contain four traversable rooms rather than a two-node label;
- three exclusive clean-room sects, three mentors and ten relationship NPC definitions;
- fifteen authored regional quests across courier, investigation, supply, hunt, escort and training-trial archetypes; every quest owns a structurally distinct per-quest DAG whose reachable nodes drive real movement, mutually exclusive or skippable route choices, persisted consequence flags, deadline failure/retry and Direct/Diplomatic/Resourceful outcomes; all 45 approaches run route authority and Direct paths run the actual encounter rather than inserting a cleared flag;
- seven RPG encounter definitions, encounter-specific original combat logs, telegraphed enemy move trees, bleed/exposure/guard/stagger states and three techniques per sect; the player freely selects primary or secondary techniques during combat, can form a combo, receives typed intent-counter bonuses and keeps independent persistent mastery;
- seven attributes, three origins, three growth paths, three mastery titles and fifteen skill definitions;
- twenty-two shop/economy items, eight crafting recipes, equipment durability and repair;
- world time, stamina, rations, water, injuries, trust, rank, journal and route planning; all ten NPCs move between work/civic/rest rooms, expose five relationship stages plus three dialogue intents, and participate in persisted pairwise bonds, work output, social events and memories; autonomous produce/migrate/ally/conflict/publish-task goals now alter rooms, production, dialogue, relationships and world flags rather than being write-only;
- four persistent regional markets with distinct stock/demand, inventory/production/consumption/quest/social-event effects and multi-leg inter-region caravans; caravans occupy real world rooms along authored routes, retain integrity/risk/incidents and can be visibly escorted or intercepted before delivery;
- three independent five-quest chapters with chapter-specific protagonists and playable scene rooms; each chapter requires two authored testimony/confrontation beats before its irreversible choice, and all five endings continue through four-beat playable epilogues with fifteen ending-specific follow-up scenes plus a closing beat into persistent post-ending world state;
- three atomic save slots and schema migration for existing saves/settings.

The RPG layer uses only clean-room mechanics study of 白金英雄坛说. No source code, text, maps, NPC/task tables, art, music or proprietary data is copied.

### RTS

- ten authored 40x24 maps: four campaign missions plus Iron Delta, Night Watch Crossing, Glass Basin, Ember Orchard, Salt Marsh Divide and Cinder Crown Siege skirmishes;
- two typed factions with twelve unit archetypes;
- ten structure definitions and ten technology definitions, all mapped into authoritative runtime kinds/jobs and filtered by the selected player faction;
- authoritative worker logistics, shared resource nodes, supply, power, construction, repair, production, research and upgrades for both sides: player and enemy jobs are one side-tagged `SimJob` model; both sides now use one worker movement/gather/cargo/return function, one source-checked authority-job command inlet, one job progress/completion scheduler and the same builder/site executor. Side-specific unit/technology effects and AI strategic selection remain adapters/controllers rather than second job authorities;
- fog/recon, control groups, queued orders, formations, stance/patrol/stop, veterancy and deterministic congestion recovery;
- typed escort, defend, extract, destroy and capture objectives;
- Story / Standard / Veteran pressure and deterministic adaptive AI;
- independent title-menu skirmish setup with map, faction matchup, starting-resource, Objective/Score/Annihilation and selectable deterministic-seed configuration, plus durable loot and normal one-time RPG settlement; the campaign gate remains an additional in-world entry;
- twelve typed unit abilities and ten distinct structure functions are executed by the authority rather than existing as names or stat-only catalog rows;
- battle orders export to chunk-hashed `trnm_battle_replay_v2`, automatically migrate a verified v1 file in the client path, retain up to 65,536 orders, persist hashed seek checkpoints, save/load independently verified disk chunk directories and expose pause/play, checkpoint seek, 1x/2x/4x/8x plus a free W/A/S/D camera; the long-match gate verifies 1,200 orders across three chunks, over twice the legacy window;
- the balance gate runs 64 terminal-oriented samples across four canonical balance YAML maps, faction swaps, spawn swaps and four meaningful simulation seeds while exercising player/enemy spending, harvesting, production and research; it reports faction win delta, resource-efficiency delta, technology delta and terminal counts by map in addition to pressure/time metrics;
- a real authored-map Standard Annihilation path uses unmodified campaign growth attributes, sustained harvesting, supply, production, research, recon, field aid and typed anti-structure abilities to destroy the actual enemy force/base, verifies replay, settles once and returns through the real client debrief key path; K then M/T/Y/U/I then Enter remains covered as the complete setup/deploy chain;
- the atlas manifest loads an original transparent 8x3 identity atlas with twelve unique unit frames and ten unique structure frames. Every identity now also receives deterministic family-specific breathing/recoil geometry motion while retaining the existing six/five base sprite rows; this improves live distinction but is still not a claim of 22 wholly independent hand-drawn frame sets.

## Stable contracts

- `trnm_campaign_save_v1`, schema revision 12;
- `term_exchange_protocol_v2` / `term_exchange_backend_v2`;
- `trnm_battle_seed_v8`;
- `trnm_battle_result_v2`;
- `trnm_settlement_receipt_v1`;
- `trnm_rts_sim_v16` / `trnm_rts_sim_checkpoint_v16`;
- `trnm_player_settings_v2`.

## Product shell

- title NEW/LOAD/CONTINUE, independent slots, corrupt-slot isolation and resume guard;
- authoritative pause, journal, progressive guide and character identity confirmation;
- low motion, input mode, three control-scheme profiles, subtitles/high contrast and live master-volume control;
- desktop installer assets under `packaging/` and `scripts/install_trnm_desktop.sh`;
- deterministic performance matrix at `scripts/check_trnm_perf_matrix.sh`.

Revision 12 separates local soft credits, CEX wallet credits, bound items,
tradeable items and ephemeral RTS resources. It persists account binding,
wallet snapshot, a bounded economic-intent outbox, a separate priority
compensation lane, verified receipts, explicit `ValueEvent` payout policies,
idempotency keys, dead letters, trade lifecycle and reconciliation cursor.
Offline play uses `OfflineLocalEconomyBackend`; connected play sends the same
typed intents to CEX. Quest, chapter, ending, battle and future trade values
are recorded with `LocalSoftOnly`, `WalletOnly` or explicit `DualTrack`
semantics; only `DualTrack` deliberately issues both local and wallet value.
Battle wallet issuance requires CEX-verified `ServerSignedValueEntitlementV1`
and is transactionally capped at 100 credits per event and 300 per UTC budget
day; positive `CompleteContract` is rejected and local soft credits are
permanently non-convertible.
Connected tradeable market purchases require explicit buyer and seller ledger
accounts and use buyer Reserve -> escrow hold -> atomic seller commit. Seller
proceeds remain reserved through a reversible payout window. Refund
and chargeback use the priority lane, refund held escrow or reverse a committed
trade, and roll back delivered inventory before compensation. Recoverable
network/ledger failures hold progression and survive save reload; malformed or
mismatched receipts fail closed.
Connected campaign and intent identifiers are deterministically scoped by the
bound CEX account, so different players' default local save names cannot
collide in the global idempotency ledger.

The client exposes local/wallet balances, pending intents, priority
compensations, value events, verified receipts and dead letters. `Ctrl+F7`
binds/reconciles with a player/account/device-scoped signed session from
`TRNM_CEX_*`; the distributable client no longer carries the shared CEX entry
token. Connected market
purchase also requires `TRNM_CEX_MARKET_ACCOUNT_ID`. High-frequency regional
markets, caravans, NPC production and RTS resources remain local deterministic
simulation and do not call CEX per tick. Public player listings remain gated;
the current path is a trusted system market, not an open player market.

The native client has a real buffered audio pipeline with two project-owned procedural WAV loops: Mirror City ambience and Signal battle pulse. Town and battle switch cleanly; ending/epilogue play uses a dedicated layered mix of both sources, and F8 updates all live players immediately. These are an original functional score system; additional composed cues, richer effects and a final mastered mix remain content work.

The control profiles alter live RTS input: Classic uses Q/W/E/R for move/attack/harvest/hold, Left Handed uses A/S/D/F, and Arrow Grid uses the four arrow keys (with WASD camera pan). They are not display-only labels.

## Completion wording and later scopes

- the 2026-07-11 finite v1 checklist is a completed historical acceptance baseline;
- the broader historical ambition of a deeply simulated Jianghu-like RPG plus a feature-complete RTS has no honest fixed 100% endpoint; current claims must enumerate implemented systems and remaining scopes instead of converting that ambition into a percentage;
- human observation and non-developer sessions are post-v1 feedback for usability and balance, not a software completion gate;
- final composed soundtrack, richer effects and mix: pending (basic original playback is complete);
- installer smoke across target distributions: pending;
- public beta/commercial launch and networking: out of current scope.

## Current local evidence

- six-crate unit/integration/E2E suite: 121/121 passing (41 Campaign, 19 First Contact, 13 RPG, 8 protocol, 31 RTS, 9 closed-loop E2E); the authored client regression begins at a default new save, uses real setup/deploy keys and authoritative orders to win all four campaign battles, then drives all fifteen quests through all three approaches, chapter scenes and a four-beat ending epilogue without directly inserting prologue flags;
- CEX full workspace: 351 passing with 16 detached-runtime black-box probes explicitly ignored; `consumer-entry-api` is 161/161. The persistent cross-process gate additionally proves new accounts, reward exactly-once, byte-identical replay across ledger/consumer restart, held-escrow refund, committed chargeback, wallet/cursor recovery and PostgreSQL uniqueness;
- workspace Clippy with `-D warnings`: passing;
- product boundary: green (6 game / 12 platform / legacy working tree absent); CEX depends on `trnm-economy-protocol` and no longer depends on removed `trnm-world-api`, `trnm-world-domain` or `trnm-world-projection` crates;
- release build and desktop installer smoke: passing;
- current X230 warm-cache matrix after explicit `--no-run` client-harness prewarming: RPG 0.69 s / 75 MiB, Campaign 0.54 s / 75 MiB, full 19-test First Contact package 88.41 s / 117 MiB, 64-sample RTS simulation 81.58 s / 431 MiB, new-save client journey 41.80 s / 118 MiB, Standard Annihilation 78.09 s / 116 MiB, authored-map adapter 12.94 s / 117 MiB, closed loop 21.27 s / 390 MiB and incremental release build 0.87 s / 118 MiB. Every row remains below the explicit 90-second / 4-GiB bound. The former 3.41-GiB client figure was rustc/linker RSS from compiling the test harness on the first measured row, not game-runtime memory; the gate now separates compilation from runtime instead of misreporting it;
- release client service: active with a viewable native window after restart.
- persistent CEX recovery: PostgreSQL WAL archival, a physical 8.6-GiB base
  backup, named-restore-point PITR and writable same-host promotion are proven;
  this is not multi-host quorum/fencing or regional HA;
- receipt projection and seller payout maintenance run from a five-minute
  timer against the PostgreSQL source of truth.

The first clean release rebuild after adding the native rustls CEX client took 9m40s under the service host's constrained X230 environment; that is a developer compile cost, not installed-game startup. These are local-machine facts, not substitutes for the pending human session or a multi-distribution performance/installer matrix.

## Verification entry points

```bash
scripts/check_trnm_game_product.sh
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets -- -D warnings
scripts/check_trnm_perf_matrix.sh
cargo build --manifest-path trillionnium/Cargo.toml --release -p trnm-first-contact
```

Historical World review/evidence documents from July 7-9 are under `docs/archive/world-review-2026-07/`; they are not current gameplay truth.
