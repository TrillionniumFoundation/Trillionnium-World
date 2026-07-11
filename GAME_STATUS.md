# TRNM Game Status

Updated: 2026-07-11

This is the one-page status source for the current native RPG + real-time-strategy product. The older finite checklist in `docs/development/trnm-deep-rpg-complete-rts-v1-dod.md` is retained as a completed historical baseline, not as a claim that the broader deep-RPG + complete-RTS vision is 100%. This does not grant blockchain, CEX, Android, multiplayer or public-launch credit.

The current depth-pass acceptance contract is `docs/development/trnm-authored-rpg-symmetric-rts-v2-dod.md`.

## Product boundary

- Product workspace: five crates in `trillionnium/Cargo.toml`.
- Native client: `trnm-first-contact`.
- RPG/world vocabulary: `trnm-rpg-core`.
- Campaign, save, progression and settlement authority: `trnm-campaign-core`.
- Player-order contract: `trnm-rts-protocol`.
- Bevy-free deterministic battle authority: `trnm-rts-sim`.
- Historical legacy game workspace: absent; see `docs/archive/frozen-legacy-final-index-2026-07-11.md`.
- CEX adapter: retired historical reference, not a current product dependency.

## Current authored scope

### RPG

- twenty connected rooms across Mirror City, Signal Road, Glass Basin and the Ashen Fringe, with story locks and authoritative route planning; Glass Basin and Ashen Fringe each contain four traversable rooms rather than a two-node label;
- three exclusive clean-room sects, three mentors and ten relationship NPC definitions;
- fifteen authored regional quests across courier, investigation, supply, hunt, escort and training-trial archetypes; every quest has its own condition graph, resolution/failure text, deadline, failure/retry state and Direct/Diplomatic/Resourceful consequences with trust, item, encounter and route-evidence gates;
- seven RPG encounter definitions, encounter-specific original combat logs, telegraphed enemy moves, bleed/exposure/guard/stagger states and three selectable techniques per sect with momentum/cooldown authority;
- seven attributes, three origins, three growth paths, three mastery titles and fifteen skill definitions;
- twenty-two shop/economy items, eight crafting recipes, equipment durability and repair;
- world time, stamina, rations, water, injuries, trust, rank, journal and route planning; all ten NPCs move between work/civic/rest rooms, expose five relationship stages plus three dialogue intents, and participate in persisted pairwise social events and memories;
- inventory/production/consumption/quest/social-event-driven market stock and demand, buy/sell pricing, a browsable shop/craft surface and owned-equipment selector;
- three atomic save slots and schema migration for existing saves/settings.

The RPG layer uses only clean-room mechanics study of 白金英雄坛说. No source code, text, maps, NPC/task tables, art, music or proprietary data is copied.

### RTS

- eight authored 40x24 maps: four campaign missions plus Iron Delta, Night Watch Crossing, Glass Basin and Ember Orchard skirmishes;
- two typed factions with twelve unit archetypes;
- ten structure definitions and ten technology definitions, all mapped into authoritative runtime kinds/jobs and filtered by the selected player faction;
- authoritative worker logistics, shared resource nodes, supply, power, construction, repair, production, research and upgrades for both sides: enemy workers are real `SimUnit` actors that carry cargo, enemy buildings are normal destructible `SimStructure` targets, and production pauses when workers/power/supply prerequisites fail;
- fog/recon, control groups, queued orders, formations, stance/patrol/stop, veterancy and deterministic congestion recovery;
- typed escort, defend, extract, destroy and capture objectives;
- Story / Standard / Veteran pressure and deterministic adaptive AI;
- independent title-menu skirmish setup with map, faction matchup, starting-resource and Objective/Score/Annihilation configuration, plus durable loot and normal one-time RPG settlement; the campaign gate remains an additional in-world entry;
- twelve typed unit abilities and ten distinct structure functions are executed by the authority rather than existing as names or stat-only catalog rows;
- battle orders can be exported as a hash-verified replay; a 24-match four-map x faction-swap x three-seed automated matrix executes the real simulation and enforces a bounded faction-pressure band;
- the atlas manifest exposes twelve roster-specific unit identities and ten structure-specific identities with distinct runtime palettes, while retaining six/five base animation rows as original authored pixel sources.

## Stable contracts

- `trnm_campaign_save_v1`, schema revision 5;
- `trnm_battle_seed_v8`;
- `trnm_battle_result_v2`;
- `trnm_settlement_receipt_v1`;
- `trnm_rts_sim_v12` / `trnm_rts_sim_checkpoint_v12`;
- `trnm_player_settings_v2`.

## Product shell

- title NEW/LOAD/CONTINUE, independent slots, corrupt-slot isolation and resume guard;
- authoritative pause, journal, progressive guide and character identity confirmation;
- low motion, input mode, three control-scheme profiles, subtitles/high contrast and live master-volume control;
- desktop installer assets under `packaging/` and `scripts/install_trnm_desktop.sh`;
- deterministic performance matrix at `scripts/check_trnm_perf_matrix.sh`.

The native client has a real buffered audio pipeline with two project-owned procedural WAV loops: Mirror City ambience and Signal battle pulse. Campaign mode switches the active loop, and F8 updates both live audio players immediately. These are a functional original baseline; a fully composed soundtrack, richer sound effects and final mix remain future content work.

The control profiles alter live RTS input: Classic uses Q/W/E/R for move/attack/harvest/hold, Left Handed uses A/S/D/F, and Arrow Grid uses the four arrow keys (with WASD camera pan). They are not display-only labels.

## Completion wording and later scopes

- the 2026-07-11 finite v1 checklist is a completed historical acceptance baseline;
- the broader historical ambition of a deeply simulated Jianghu-like RPG plus a feature-complete RTS has no honest fixed 100% endpoint; current claims must enumerate implemented systems and remaining scopes instead of converting that ambition into a percentage;
- human observation and non-developer sessions are post-v1 feedback for usability and balance, not a software completion gate;
- final composed soundtrack, richer effects and mix: pending (basic original playback is complete);
- installer smoke across target distributions: pending;
- public beta/commercial launch and networking: out of current scope.

## Current local evidence

- five-crate unit/integration/E2E suite: 95/95 passing;
- workspace Clippy with `-D warnings`: passing;
- product boundary: green (5 game / 12 platform / legacy working tree absent);
- release build and desktop installer smoke: passing;
- current X230 validation matrix after the authored/symmetric depth pass: RPG 3.18 s / 260 MiB, campaign 9.33 s / 524 MiB, RTS 40.43 s / 386 MiB, closed loop 13.29 s / 70 MiB, incremental release build 65.99 s / 743 MiB;
- release client service: active with a viewable native window after restart.

The first clean release rebuild after changing the Bevy/audio feature graph took 12m08s under the service host's constrained X230 environment; that is a developer compile cost, not installed-game startup. These are local-machine facts, not substitutes for the pending human session or a multi-distribution performance/installer matrix.

## Verification entry points

```bash
scripts/check_trnm_game_product.sh
cargo test --manifest-path trillionnium/Cargo.toml --workspace --all-targets
cargo clippy --manifest-path trillionnium/Cargo.toml --workspace --all-targets -- -D warnings
scripts/check_trnm_perf_matrix.sh
cargo build --manifest-path trillionnium/Cargo.toml --release -p trnm-first-contact
```

Historical World review/evidence documents from July 7-9 are under `docs/archive/world-review-2026-07/`; they are not current gameplay truth.
