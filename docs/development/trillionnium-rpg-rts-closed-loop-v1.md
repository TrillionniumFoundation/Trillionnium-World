# Trillionnium RPG -> RTS -> RPG Closed Loop v1

Status: canonical product contract as of 2026-07-11. `GAME_STATUS.md` is the concise current status and open-gate view.

## Product Definition

Trillionnium is an RPG/real-time-strategy hybrid. Its RPG layer may use
clean-room mechanics study of 白金英雄坛说, but no source code, text, NPC,
quest table, data, or art may be copied. The first playable closure is:

```text
Mirror Square
  -> meet Street Compass Sifu
  -> train Basic Unarmed
  -> equip Route Guard Staff
  -> choose hero + three persistent companions
  -> accept First Contact
  -> play the authored 40x24 RTS mission
  -> victory / defeat / withdrawal
  -> stage BattleResult
  -> idempotent settlement
  -> return to Mirror Square
  -> persist XP / skill XP / loot / reputation / injuries / quest state
  -> unlock repeatable Aftershock Patrol
  -> consume the persisted growth in a stronger BattleSeed
  -> fight, settle and continue the campaign loop
  -> complete the typed Signal Road quest chain
  -> unlock the persistent Relay Quarter world room
  -> escort a supply convoy, defend its generator and extract on a third original map
  -> unlock the outer Signal Road and retain the repeatable patrol loop
  -> counterattack on the fourth original Mirror Siege map
  -> break the siege, reclaim Mirror Gate and persist the result
  -> select Iron Delta, Night Watch Crossing, Glass Basin or Ember Orchard as deterministic skirmishes
  -> return their faction loot and equipment materials to the RPG economy
```

## Authority Boundaries

| Surface | Owner | Rule |
| --- | --- | --- |
| RPG/world primitives | `trnm-rpg-core` | Lightweight attributes, character/inventory vocabulary and data-driven world graph/transition rules used by the product. |
| Campaign/save/progression | `trnm-campaign-core` | Sole authority for RPG mutation and settlement. |
| Frame-order contract | `trnm-rts-protocol` | Lightweight player-order validation and deterministic stream contract. |
| Battle simulation | `trnm-rts-sim` | Bevy-free, two-dimensional, map-aware simulation consuming `RtsFrameOrder` as its only player input. |
| Native presentation/input | `trnm-first-contact` | Consumes `BattleSeedV1`; may only emit `BattleResultV1`. |
| Authored map/art | `assets/first_contact` | Eight canonical original 40x24 campaign/skirmish maps and PNG atlases. |
| Historical implementation | `docs/archive/frozen-legacy-final-index-2026-07-11.md` | Removed from the current checkout; recoverable by an explicit historical Git worktree only. |

## Stable Contracts

- `trnm_campaign_save_v1`
- `trnm_battle_seed_v8`
- `trnm_battle_result_v2`
- `trnm_settlement_receipt_v1`
- `trnm_rts_sim_v14`
- `trnm_rts_sim_checkpoint_v14`

Campaign persistence remains `trnm_campaign_save_v1` with schema revision 7.
Revision 7 adds branch-driven per-quest DAG navigation, explicit ending state,
primary/secondary technique loadouts, four regional markets and deterministic
inter-region logistics while preserving revision-6 migration paths.

`BattleSeedV1` binds campaign revision, battle id, map/rules version, four
persistent party members, spawn slots, skills, typed equipment modifiers,
injuries, character origin, earned mastery title, a typed mission/objective
sequence, expedition preparation/readiness, mapped RTS stats, and a SHA-256
payload hash.

`BattleResultV1` binds the seed hash, terminal outcome, every party member's
HP/status/XP, loot, resource and reputation deltas, world flags, elapsed ticks,
and the final deterministic snapshot hash.

Settlement is two-phase and crash safe:

1. atomically save `post_battle_pending` with the validated result;
2. apply progression once and atomically save `SettlementReceiptV1`;
3. on restart, finish a pending settlement;
4. reject a changed payload for an already settled battle id;
5. return zero deltas for an exact duplicate result.

## RPG -> RTS Mapping

- physique + resolve -> HP and armor;
- force -> base damage;
- agility -> move speed, attack interval and evasion;
- insight + resolve -> energy and ability range, both consumed by signature abilities;
- skill rank -> bounded skill-power multiplier;
- equipment -> typed modifiers keyed by item id, never parsed display text;
- origin + build path + earned mastery -> conditional equipment affixes;
- injury -> bounded HP and movement penalties;
- party member -> named persistent unit bound to `party_0..party_3` spawn slots.

Every non-material equipment catalog entry has an explicit typed modifier.
The invariant is exhaustive, so a crafted/catalog ID cannot silently miss its
BattleSeed effect.

## Deeper RPG and symmetric skirmish authority

- all fifteen regional quests own a structurally distinct typed DAG whose
  currently reachable nodes drive route choice, plus authored approach prose,
  real encounter execution, failure recovery and relationship/economy effects;
- the twenty-room world includes four-room Glass Basin and Ashen Fringe
  regions plus three independent chapter decisions and five ending states;
- NPC schedules produce bounded pairwise social-event memories; persisted bond
  and work output alter goals, production, four regional markets and logistics;
- each sect exposes an alternating primary/secondary technique loadout with
  separate mastery, telegraphed enemy moves and typed combat statuses;
- both RTS sides use `SimUnit` workers, shared resource nodes, cargo return,
  builder-bound sites, `SimStructure` buildings and one side-generic structure
  function authority alongside power, supply, production and research jobs;
- enemy buildings are explicit selectable attack targets; proximity cannot
  damage a base;
- all twelve roster abilities have individual authoritative effects;
- `trnm_battle_replay_v2` chunk-hashes up to 65,536 frame orders, migrates v1,
  exposes a title replay browser and verifies the final snapshot hash; the
  terminal-oriented balance gate runs four real YAML maps, both faction and
  spawn assignments, and three gameplay-affecting seeds through live authority.

The authored map's four selected player records are presentation spawn slots;
they are no longer hard-coded RPG character identities.

## Player Controls

Town:

- `F1`: open the title/save-slot shell; `F4`: open the campaign journal;
  `Esc`: pause gameplay

- `1`: Mirror Square
- `2`: mentor hall
- `3`: expedition gate
- `4`: Relay Quarter after the two-mission Signal Road chain
- `5/6/7/8/9/0/-/=`: Cistern Ward, Night Watch Post, Iron Workshop Gate,
  Market Wind Pavilion, Lantern Infirmary, Archive Steps, Caravan Yard and Outer Signal Road
- `Shift+1/2/3/4/5/6/7/8`: Glass Basin Wayhouse, Deep Relay, Moon Bridge, Ember Orchard Edge, Glass Reed Marsh, Basin Observatory, Ash Beacon Field and Cinder Refuge after their story locks open
- `T`: talk to the scheduled NPC; `Shift+T`: cycle ask-for-work / offer-help / share-news dialogue intent
- `L`: cycle Iron Guard / Wind Step / Inner Flame training path
- `K`: buy one capped mentor training session
- `E`: cycle Guard / Raider / Mystic typed loadouts
- `Z/X/C`: independently cycle companion slots one/two/three
- `P`: cycle currently valid four-person presets
- `Y`: run the deterministic mentor sparring bout after training
- `U`: recruit Brann in Relay Quarter after reaching eight trust
- `H`: consume a Field Tonic or pay the field clinic to reduce injuries
- `G`: equip the recovered Relay Core relic after a victory
- `A`: cycle a permanent stat-allocation preview; `S`: confirm and consume one
  growth point; `D`: cancel without spending; `O`: select origin before mentor
  commitment; `Q`: complete the selected path mastery; `V`: cycle earned titles
- `J`: begin the Signal Road typed RPG encounter from Relay Quarter (or the
  Gate Warden route); during it use `J/R/K/I/Esc` for attack/defend/sect-technique/item/withdraw
- `B`: start/advance Cistern Relief; at its branch use `N` to reinforce or `M`
  to evacuate; its supply step is completed at the expedition gate
- `R` at the expedition gate: cycle immediate/rested/supplied/shortcut preparation
- `F6` at the expedition gate: cycle Story / Standard / Veteran before acceptance
- `F`: accept mission / deploy
- `F9/F10`: accept and advance the authored regional quest available at the current NPC room; `R` away from the expedition gate cycles Direct/Diplomatic/Resourceful resolution
- `T/K` in a regional mentor hall: commit to one of three sects and train its prerequisite skill tree
- `Shift+K` / `Ctrl+Shift+K`: select the primary / secondary sect technique
- `F11`: cycle shop/crafting/equipment; `Shift+F11`: buy or craft; `Ctrl+F11`: sell at daily demand price; `F12`: equipment repair

Title/pause shell:

- `1/2/3`: select independent campaign slot A/B/C
- `N`: create a new campaign; press a second time to explicitly confirm overwrite
- new character: `C` cycles the persistent display name; `Enter` confirms it
- `Enter`: load/continue, then pass the resume guard before gameplay is revealed
- `K`: open independent skirmish setup for the selected existing slot; then `M/T/Y/U/I` select map/factions/resources/victory/deterministic seed and `Enter` deploys
- `P`: open the selected slot's last verified chunked replay browser
- `F2`: low-motion mode; `F3`: hybrid/keyboard-only/mouse-only input mode
- `F5`: subtitles/high contrast; `F7`: control-scheme profile; `F8`: live master volume for the town/battle Bevy audio sinks
- `Esc`: resume from pause; settings are profile-scoped rather than character-scoped

Battle:

- `Q`: advance
- `W`: assault
- `E`: harvest
- `R`: hold
- `A`: activate selected units' energy/cooldown/range-bound signature abilities
- `S`: spend 20 field resources on Field Aid for selected units
- `H`: cycle all faction-compatible structures; `D`: build the selected structure inside an authoritative build radius
- `C`: spend 10 field resources on a recon sweep that reveals authoritative fog
- `Z`: cycle the full six-unit faction roster; `V`: queue the selection
- `T`: cycle all faction-compatible technologies; `B` queues research and `N` queues upgrades
- `X`: withdraw
- `0`: select all; `Ctrl+1..9`: assign a control group; `1..9`: recall it;
  double-tap the group number to focus the camera on living members
- hold `Shift` while issuing move/attack/harvest/hold to append a queued order
- `Delete`: cancel the newest queued tactical order
- `U`: cancel the first production job with one-time half refund
- `Y`: pause/resume the first job; `O`: promote the last job; `M`: set its rally point
- mouse drag: select one or more party units; click the minimap to retarget and center the camera
- `F`: cycle wedge / line / column formation during battle
- `G`: cycle hold-fire / guard / aggressive stance; `P`: patrol between the
  current position and target; `Space`: stop and clear the selected queue
- `Tab`: cycle unit/resource/objective targets
- `I/J/K/L`: move a free target across passable map tiles
- arrow keys: camera

Debrief:

- `Enter`: return to Mirror Square after the atomically saved settlement.

Current authored content expansion:

- twenty-room four-region world, three exclusive sects, three mentors and ten moving NPC definitions;
- fifteen regional quests, seven encounter definitions and original combat captions;
- fifteen skills, twenty-two economy items, eight recipes, durability and repair;
- two factions, twelve unit archetypes, ten structures and ten technologies;
- Iron Delta, Night Watch Crossing, Glass Basin and Ember Orchard join the four campaign maps as independently selectable skirmishes.

## M0-M4 Implementation State

- M0: complete. Versioned contracts, payload hashes, validation failures and
  atomic save tests are in `trnm-campaign-core`.
- M1: complete. Twenty-room RPG, mentor dialogue/training, equipment, party,
  quest acceptance, campaign UI and atomic save are in the default client.
- M2: complete. `BattleSeedV1` embeds the authored 40x24 navigation projection;
  `RtsFrameOrder` directly drives deterministic 2D pathfinding, occupancy,
  selection, free targets, resources, active map-aware enemies, relay pressure,
  abilities, victory, defeat, withdrawal, casualties and checkpoints.
- M3: complete. Battle result staging, crash recovery, idempotent settlement,
  debrief/return UI, XP, skill XP, loot, reputation, injuries, quest state and
  durable reload are connected.
- M4 software gates: complete. Nine closed-loop E2E cases cover victory,
  defeat, withdrawal, mid-battle crash, mid-settlement crash, duplicate result,
  two consecutive victories with reload between First Contact and the
  repeatable Aftershock Patrol, the convoy chain and independent configurable
  skirmish settlement.
- Gameplay P0: complete. Training is paid and capped; withdrawal pays zero XP
  and resources; defeat rewards are bounded; harvested resources settle only
  on victory; loot can be consumed for healing or equipped as a typed relic;
  single-order victories are rejected.
- Gameplay P1: complete. Seven candidates provide three party compositions,
  three mentor paths and three equipment loadouts; signature skills consume
  energy/range/cooldown; harvest and ability-rush routes are both viable. The
  first mission requires approach, contact, relay assault, two deterministic
  counterattack waves and hold/capture. Field resources can be spent on aid or
  fortification. Tested routes require 8-12 consequential orders and complete
  in 3-5 simulated minutes; four-order idle play cannot win.
- Gameplay P2: complete. The root workspace contains only five product crates;
  platform has 12 crates and the removed 14-crate legacy workspace is absent
  from the current checkout. The product uses only the lightweight RPG and
  order-protocol cores; historical sources require an explicit Git worktree.
  The canonical player, map, save and order authorities are singular.
- Legacy extraction P0: complete. The twenty-room, four-region world uses a
  validated data-driven graph. The expedition gate is mentor-locked,
  non-adjacent travel is rejected and frontier regions are story-locked.
- Legacy extraction P1: complete. Typed `QuestDefinition`, conditions, rewards
  and persisted story steps drive an original Signal Road arc. First Contact
  and Aftershock use distinct authored 40x24 maps; winning both and returning
  to town unlocks Relay Quarter, including after save/reload.
- Legacy extraction P2: software-complete. Camera clamps and minimap mapping are
  parameterized by map/viewport size; mouse drag selection, minimap targeting
  and wedge/line/column orders are wired to the live client. Human five-second
  evidence remains pending and is not claimed by these software tests.
- Legacy extraction P3: complete for the expanded playable rules. The protocol
  and deterministic sim own assign/append/remove/recall control groups, Shift
  order queues, exact cancellation, production pause/resume/promote/refund and
  validated rally points. Current/explored fog is authoritative: hidden enemy
  ids are rejected and recon reveals deterministic map regions without leaking
  them through the client. Production now has support/medic branches, logistics
  and optics research, arms and armor upgrades, plus escalating enemy targeting.
  RPG preparation now includes free companion-slot selection, typed NPC trust,
  faction rank, Relay Smith recruitment and deterministic mentor sparring.
- Growth/economy P0: complete. Growth-point preview/confirm/cancel is atomic,
  one-time and reload-safe. Force and agility allocations produce observably
  different hashed BattleSeeds. Vanguard/Windrunner/Artificer titles affect
  combat, a shortcut/encounter route, NPC trust or clinic/build prices.
- Growth/economy P1: complete. Resource nodes deplete, workers carry cargo back
  to the command post, and only deposited resources are spendable. Structures
  own supply, power, build radius, prerequisites and repair; low power pauses
  jobs until generation recovers, all inside deterministic checkpoints.
- Growth/economy P2: complete. Typed stance, patrol and Stop orders are live;
  kills grant bounded veterancy which changes combat and returns through
  `BattleResult` into each persistent party member.
- Growth/economy P3: complete. The persistent Signal Road encounter supports
  attack, defend, real item consumption and withdrawal. Victory/defeat writes
  loot, injury, reputation and world flags to the campaign aggregate.
- Route/mission P0: complete. `WorldRoutePlan` performs lock-aware stable BFS,
  explicit unreachable diagnostics and ordered multi-waypoint planning. Town UI
  shows the next exit or the exact blocked reason for the current story step.
- Route/mission P1: complete. `MissionDefinition` and `ObjectiveKind` own typed
  objective sequences. The original Convoy Exodus map runs Escort -> Defend ->
  Extract rather than reusing the relay phases, and has a full settlement/reload E2E.
- Route/mission P2: complete. Mirror Ward, Workshop Kin and Signal Runner origins
  combine with three growth paths. Growth selects a path only; a typed mastery
  challenge earns the title. Conditional affixes and all 3x3 combinations produce
  distinct hashed seeds and observable combat stats.
- Route/mission P3: complete. Movement owns typed intents, per-tick tile
  reservations, stable blocked priority, bounded yield/replanning and checkpoint
  hashes. The eight-actor congestion regression proves no overlap and recovery
  after a blocked chokepoint opens.
- Product shell P0: complete. Three atomic save slots have independent campaign
  and battle-checkpoint paths, metadata, explicit overwrite confirmation,
  corrupt-slot isolation and a title NEW/LOAD/CONTINUE flow. A resume guard
  masks restored state. Pause stops authoritative ticks, orders, selection and
  camera input. Low-motion and real input-mode settings persist outside slots.
- Quest-chain P1: complete. Generic typed chain/node/condition/reward/branch
  definitions drive the original Cistern Relief task. Survey, supply and
  branch steps use world rooms and produce distinct credit/reputation/trust/flag
  outcomes which survive reload.
- Readiness P2: complete. World time, stamina, rations and water are campaign
  state. Immediate, rested, supplied and shortcut expeditions consume different
  resources/time and bind observable HP/energy/movement/starting-resource
  differences into the BattleSeed. Battle time and victory recovery settle back.
- Adaptive AI P3: complete. The deterministic sim records bounded typed
  observations, budget and decisions. Scout, economy raid, tech counter,
  objective defense, convoy interdiction and assault goals change target scoring;
  invalid commands do not mutate the replay and checkpoints preserve it exactly.
- Human-evidence P0: procedure-complete, observation-pending. A current runbook,
  three-observer form, 10-15 minute session form and executable packet checker
  are present. They deliberately remain pending until real people participate.
- Guided campaign P1: complete. A typed journal derives Signal Road, Cistern
  Relief and mastery entries from authoritative state; the town guide advances
  through mentor, training, equipment, gate, acceptance and deployment steps.
- Character identity P2: complete. New slots require explicit confirmation of
  one typed display-name preset; the canonical character and party hero stay
  synchronized across atomic reload. Existing `CharacterOrigin` remains the
  sole gameplay origin system.
- Original content P3: complete. Mirror Siege is a fourth original authored
  map with a five-unit enemy deployment and breach/destroy/capture objectives.
  Story, Standard and Veteran scale enemy pressure and deterministic AI cadence.
  Its full convoy-to-siege RPG -> RTS -> RPG settlement survives reload.

## Repeatable Campaign Loop

- A First Contact victory sets `first_contact_secured` and unlocks Aftershock
  Patrol rather than terminating progression.
- Aftershock is repeatable, scales the relay guard and reinforcement waves,
  records its own completion count and uses a distinct authored terrain layout.
- The first Aftershock unlocks Convoy Exodus; its escort/defend/extract victory
  sets `convoy_exodus_secured` and `outer_signal_road_open` before patrol repeats.
- Convoy Exodus unlocks Mirror Siege; winning the counterattack sets
  `mirror_siege_secured`, then the repeatable Aftershock loop resumes.
- Character level, companion experience, reputation, relic modifiers and
  injuries all alter the next mission seed.
- Campaign load/restart preserves the unlock, growth and next seed mapping.

Human gates remain evidence-bound and must not be fabricated:

- three independent observers must answer player, selection, objective and next
  command within five seconds;
- one real 10-15 minute session must cover town choices plus at least one 3-5
  minute mission and record input/readability/flow notes;
- `playtests/first_contact-visual-review.yaml` remains pending until those
  observations are actually recorded.
- `playtests/first-contact-human-play-session.yaml` likewise remains pending;
  `scripts/check_trnm_human_validation_packet.sh` verifies readiness, not people.

## Focused Verification

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-campaign-core --all-targets
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-rts-sim --all-targets
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-first-contact --all-targets
cargo clippy --manifest-path trillionnium/Cargo.toml \
  -p trnm-campaign-core -p trnm-rts-sim -p trnm-first-contact \
  --all-targets -- -D warnings
cargo build --manifest-path trillionnium/Cargo.toml --release -p trnm-first-contact
```

The eight closed-loop cases and gameplay exploit/resource regressions live in
`trillionnium/crates/trnm-rts-sim/tests/campaign_closed_loop.rs`.

## Workspace Boundaries

- game product: `trillionnium/Cargo.toml` (5 members);
- platform: `trillionnium/crates/platform/Cargo.toml` (12 members);
- removed legacy game: absent from the working tree; final inventory and recovery
  anchors are in `docs/archive/frozen-legacy-final-index-2026-07-11.md`;
- `scripts/run_trnm_first_contact.sh` is the only player runner. The old
  `run_trillionnium_world_bevy_client.sh` name is a compatibility delegator.

## Frozen Scope

This closure does not authorize networking, hosted/public service, Android S5,
beta/commercial launch, CEX reconnection, OpenRA compatibility, old GPL-derived
map restoration, blockchain settlement, or a new large acceptance packet.
Those remain separate future decisions; removed code must not be reconnected
from Git history as a shortcut.
