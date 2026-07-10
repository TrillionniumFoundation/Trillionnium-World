# Trillionnium RPG -> RTS -> RPG Closed Loop v1

Status: canonical local product truth source as of 2026-07-10, growth/economy/encounter P0-P3 revision.

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
```

## Authority Boundaries

| Surface | Owner | Rule |
| --- | --- | --- |
| RPG/world primitives | `trnm-rpg-core` | Lightweight attributes, character/inventory vocabulary and data-driven world graph/transition rules used by the product. |
| Campaign/save/progression | `trnm-campaign-core` | Sole authority for RPG mutation and settlement. |
| Frame-order contract | `trnm-rts-protocol` | Lightweight player-order validation and deterministic stream contract. |
| Battle simulation | `trnm-rts-sim` | Bevy-free, two-dimensional, map-aware simulation consuming `RtsFrameOrder` as its only player input. |
| Native presentation/input | `trnm-first-contact` | Consumes `BattleSeedV1`; may only emit `BattleResultV1`. |
| Authored map/art | `assets/first_contact` | Two canonical original 40x24 mission maps and PNG atlases. |
| Legacy implementation | `trillionnium/crates/legacy-game/trnm-world-bevy/src/legacy.rs` | Feature-gated frozen behavior/test reference; not reconnected wholesale. |
| Older World/RTS cores and data/evidence/online | `legacy-game/trnm-world-domain`, `legacy-game/trnm-rts-core`, `trnm-rts-data`, `trnm-rts-evidence`, `trnm-rts-online` | Frozen outside this closure. GPL-derived internal map data is not a product dependency. |

## Stable Contracts

- `trnm_campaign_save_v1`
- `trnm_battle_seed_v3`
- `trnm_battle_result_v2`
- `trnm_settlement_receipt_v1`
- `trnm_rts_sim_v5`
- `trnm_rts_sim_checkpoint_v5`

`BattleSeedV1` binds campaign revision, battle id, map/rules version, four
persistent party members, spawn slots, skills, typed equipment modifiers,
injuries, mapped RTS stats, and a SHA-256 payload hash.

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
- injury -> bounded HP and movement penalties;
- party member -> named persistent unit bound to `party_0..party_3` spawn slots.

The authored map's four selected player records are presentation spawn slots;
they are no longer hard-coded RPG character identities.

## Player Controls

Town:

- `1`: Mirror Square
- `2`: mentor hall
- `3`: expedition gate
- `4`: Relay Quarter after the two-mission Signal Road chain
- `T`: talk to mentor, or build Brann trust in Relay Quarter
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
  growth point; `D`: cancel without spending; `V`: cycle unlocked build titles
- `J`: begin the Signal Road typed RPG encounter from Relay Quarter (or the
  Gate Warden route); during it use `J/R/I/Esc` for attack/defend/item/withdraw
- `F`: accept mission / deploy

Battle:

- `Q`: advance
- `W`: assault
- `E`: harvest
- `R`: hold
- `A`: activate selected units' energy/cooldown/range-bound signature abilities
- `S`: spend 20 field resources on Field Aid for selected units
- `H`: cycle barricade / generator / workshop / supply-cache construction;
  `D`: build the selected structure inside an authoritative build radius
- `C`: spend 10 field resources on a recon sweep that reveals authoritative fog
- `Z`: switch support-drone / field-medic production; `V`: queue the selection
- `B`: research Field Logistics, then Signal Optics
- `N`: upgrade Relay Arms, then Field Armor
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

## M0-M4 Implementation State

- M0: complete. Versioned contracts, payload hashes, validation failures and
  atomic save tests are in `trnm-campaign-core`.
- M1: complete. Three-room RPG, mentor dialogue/training, equipment, party,
  quest acceptance, campaign UI and atomic save are in the default client.
- M2: complete. `BattleSeedV1` embeds the authored 40x24 navigation projection;
  `RtsFrameOrder` directly drives deterministic 2D pathfinding, occupancy,
  selection, free targets, resources, active map-aware enemies, relay pressure,
  abilities, victory, defeat, withdrawal, casualties and checkpoints.
- M3: complete. Battle result staging, crash recovery, idempotent settlement,
  debrief/return UI, XP, skill XP, loot, reputation, injuries, quest state and
  durable reload are connected.
- M4 software gates: complete. Seven closed-loop E2E cases cover victory,
  defeat, withdrawal, mid-battle crash, mid-settlement crash, duplicate result,
  and two consecutive victories with reload between First Contact and the
  repeatable Aftershock Patrol.
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
  platform has 12 crates and the frozen legacy game has 14 crates. The old
  broad World/RTS cores live only in legacy; the product uses the lightweight
  RPG and order-protocol cores.
  The canonical player, map, save and order authorities are singular.
- Legacy extraction P0: complete. The three-room town now uses a validated
  data-driven world graph. The expedition gate is mentor-locked, non-adjacent
  travel is rejected and Relay Quarter is story-locked.
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

## Repeatable Campaign Loop

- A First Contact victory sets `first_contact_secured` and unlocks Aftershock
  Patrol rather than terminating progression.
- Aftershock is repeatable, scales the relay guard and reinforcement waves,
  records its own completion count and uses a distinct authored terrain layout.
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

The seven M4 cases and gameplay exploit/resource regressions live in
`trillionnium/crates/trnm-rts-sim/tests/campaign_closed_loop.rs`.

## Workspace Boundaries

- game product: `trillionnium/Cargo.toml` (5 members);
- platform: `trillionnium/crates/platform/Cargo.toml` (12 members);
- frozen legacy game: `trillionnium/crates/legacy-game/Cargo.toml` (14 members);
- legacy monolith compilation requires explicit `--features legacy`;
- `scripts/run_trnm_first_contact.sh` is the only player runner. The old
  `run_trillionnium_world_bevy_client.sh` name is a compatibility delegator.

## Frozen Scope

This closure does not authorize networking, hosted/public service, Android S5,
beta/commercial launch, CEX reconnection, OpenRA compatibility, old GPL-derived
map promotion, blockchain settlement, or a new large acceptance packet. Those
remain separate future decisions.
