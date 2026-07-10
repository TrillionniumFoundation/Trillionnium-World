# Trillionnium RPG -> RTS -> RPG Closed Loop v1

Status: canonical local product truth source as of 2026-07-10.

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
```

## Authority Boundaries

| Surface | Owner | Rule |
| --- | --- | --- |
| RPG/world primitives | `trnm-world-domain` | Reusable attributes, character and inventory vocabulary. |
| Campaign/save/progression | `trnm-campaign-core` | Sole authority for RPG mutation and settlement. |
| Frame-order contract | `trnm-rts-core` | Validates player command ordering. |
| Battle simulation | `trnm-rts-sim` | Bevy-free deterministic HP, movement, enemy AI, combat, capture and terminal outcome. |
| Native presentation/input | `trnm-first-contact` | Consumes `BattleSeedV1`; may only emit `BattleResultV1`. |
| Authored map/art | `assets/first_contact` | Canonical original 40x24 map and PNG atlases. |
| Legacy implementation | `trnm-world-bevy/src/legacy.rs` | Frozen behavior/test reference; not reconnected wholesale. |
| Older RTS data/evidence/online | `trnm-rts-data`, `trnm-rts-evidence`, `trnm-rts-online` | Frozen outside this closure. GPL-derived internal map data is not a product dependency. |

## Stable Contracts

- `trnm_campaign_save_v1`
- `trnm_battle_seed_v1`
- `trnm_battle_result_v1`
- `trnm_settlement_receipt_v1`
- `trnm_rts_sim_v1`
- `trnm_rts_sim_checkpoint_v1`

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
- insight + resolve -> energy and ability range;
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
- `T`: talk to mentor
- `K`: train
- `E`: equip starter weapon
- `P`: select hero plus three companions
- `F`: accept mission / deploy

Battle:

- `Q`: advance
- `W`: assault
- `E`: harvest
- `R`: hold
- `X`: withdraw
- arrow keys: camera

Debrief:

- `Enter`: return to Mirror Square after the atomically saved settlement.

## M0-M4 Implementation State

- M0: complete. Versioned contracts, payload hashes, validation failures and
  atomic save tests are in `trnm-campaign-core`.
- M1: complete. Three-room RPG, mentor dialogue/training, equipment, party,
  quest acceptance, campaign UI and atomic save are in the default client.
- M2: complete. `BattleSeedV1` drives deterministic party/enemy movement, unit
  HP, active enemy attacks, relay guard, capture, victory, defeat, withdrawal,
  casualties and battle checkpoints.
- M3: complete. Battle result staging, crash recovery, idempotent settlement,
  debrief/return UI, XP, skill XP, loot, reputation, injuries, quest state and
  durable reload are connected.
- M4 software gates: complete. Six closed-loop E2E cases cover victory, defeat,
  withdrawal, mid-battle crash, mid-settlement crash and duplicate result. The
  deterministic victory path is constrained to 10-15 simulated minutes.

Human gates remain evidence-bound and must not be fabricated:

- three independent observers must answer player, selection, objective and next
  command within five seconds;
- one real 10-15 minute play session must record input/readability/flow notes;
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

The six M4 cases live in
`trillionnium/crates/trnm-rts-sim/tests/campaign_closed_loop.rs`.

## Frozen Scope

This closure does not authorize networking, hosted/public service, Android S5,
beta/commercial launch, CEX reconnection, OpenRA compatibility, old GPL-derived
map promotion, blockchain settlement, or a new large acceptance packet. Those
remain separate future decisions.
