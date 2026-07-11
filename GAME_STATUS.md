# TRNM Game Status

Updated: 2026-07-11

This is the one-page status source for the current native RPG + real-time-strategy product. The finite software-v1 scope in `docs/development/trnm-deep-rpg-complete-rts-v1-dod.md` is complete. This does not grant blockchain, CEX, Android, multiplayer or public-launch credit.

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

- twelve connected rooms in the first Mirror City / Signal Road region;
- three exclusive clean-room sects, three mentors and ten relationship NPC definitions;
- fifteen authored regional quests across courier, investigation, supply, hunt, escort and training-trial archetypes; every quest now requires its giver conversation and at least two ordered persisted route steps before encounter/settlement;
- seven RPG encounter definitions, encounter-specific original combat logs, momentum and a cooldown-gated technique;
- seven attributes, three origins, three growth paths, three mastery titles and fifteen skill definitions;
- eighteen shop/economy items, four crafting recipes, equipment durability and repair;
- world time, stamina, rations, water, injuries, trust, rank, journal and route planning; all ten NPCs have daily schedules, activities and unique low/high-trust dialogue;
- a player-browsable eighteen-item shop, four-recipe craft selector and owned-equipment selector rather than fixed one-button transactions;
- three atomic save slots and schema migration for existing saves/settings.

The RPG layer uses only clean-room mechanics study of 白金英雄坛说. No source code, text, maps, NPC/task tables, art, music or proprietary data is copied.

### RTS

- six authored 40x24 maps: four campaign missions plus Iron Delta and Night Watch Crossing skirmishes;
- two typed factions with twelve unit archetypes;
- ten structure definitions and ten technology definitions, all mapped into authoritative runtime kinds/jobs and filtered by the selected player faction;
- authoritative worker logistics, resources, supply, power, construction, repair, production, research and upgrades;
- fog/recon, control groups, queued orders, formations, stance/patrol/stop, veterancy and deterministic congestion recovery;
- typed escort, defend, extract, destroy and capture objectives;
- Story / Standard / Veteran pressure and deterministic adaptive AI;
- skirmish selection after Mirror Siege with map, faction matchup, starting-resource and Objective/Score/Annihilation configuration, plus durable loot and campaign settlement.

## Stable contracts

- `trnm_campaign_save_v1`, schema revision 3;
- `trnm_battle_seed_v8`;
- `trnm_battle_result_v2`;
- `trnm_settlement_receipt_v1`;
- `trnm_rts_sim_v10` / `trnm_rts_sim_checkpoint_v10`;
- `trnm_player_settings_v2`.

## Product shell

- title NEW/LOAD/CONTINUE, independent slots, corrupt-slot isolation and resume guard;
- authoritative pause, journal, progressive guide and character identity confirmation;
- low motion, input mode, three control-scheme profiles, subtitles/high contrast and live master-volume control;
- desktop installer assets under `packaging/` and `scripts/install_trnm_desktop.sh`;
- deterministic performance matrix at `scripts/check_trnm_perf_matrix.sh`.

The native client has a real buffered audio pipeline with two project-owned procedural WAV loops: Mirror City ambience and Signal battle pulse. Campaign mode switches the active loop, and F8 updates both live audio players immediately. These are a functional original baseline; a fully composed soundtrack, richer sound effects and final mix remain future content work.

The control profiles alter live RTS input: Classic uses Q/W/E/R for move/attack/harvest/hold, Left Handed uses A/S/D/F, and Arrow Grid uses the four arrow keys (with WASD camera pan). They are not display-only labels.

## Software-v1 completion and later scopes

- deep RPG + complete RTS finite v1 definition of done: 100%;
- human observation and non-developer sessions are post-v1 feedback for usability and balance, not a software completion gate;
- final composed soundtrack, richer effects and mix: pending (basic original playback is complete);
- installer smoke across target distributions: pending;
- public beta/commercial launch and networking: out of current scope.

## Current local evidence

- five-crate unit/integration/E2E suite: 86/86 passing;
- workspace Clippy with `-D warnings`: passing;
- product boundary: green (5 game / 12 platform / legacy working tree absent);
- release build and desktop installer smoke: passing;
- warm-cache matrix on the current X230 host: RPG 0.37 s / 70 MiB, campaign 0.34 s / 70 MiB, RTS 28.05 s / 70 MiB, closed loop 14.04 s / 70 MiB, incremental release build 0.55 s / 106 MiB;
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
