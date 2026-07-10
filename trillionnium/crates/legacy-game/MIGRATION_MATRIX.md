# Legacy Gameplay Migration Matrix

The legacy monolith is archived and feature-gated. Product mechanics were
reimplemented as focused, Bevy-free behavior with product-owned tests; no old
map data or whole-world state was reconnected.

| Legacy behavior family | Product owner | Product regression |
| --- | --- | --- |
| tile pathing and blocked terrain | `trnm-rts-sim` | authored 40x24 map route test |
| unit occupancy and map-aware pursuit | `trnm-rts-sim` | deterministic checkpoint and one-order rejection tests |
| worker cargo, resource depletion and drop-off economy | `trnm-rts-sim` + `trnm-campaign-core` | no-teleport cargo, resource conservation, victory settlement and withdrawal zero-delta tests |
| target, assign/append/remove/recall control groups | `trnm-rts-protocol` + `trnm-rts-sim` + `trnm-first-contact` | authoritative group membership, dead-member pruning and recall tests |
| shift order queue and cancellation | `trnm-rts-protocol` + `trnm-rts-sim` | typed queue id, sequential activation and exact cancellation tests |
| production lifecycle and rally | `trnm-rts-sim` | pause/resume, promote, blocked-rally rejection and one-time half-refund tests |
| building, supply, power, prerequisites and repair | `trnm-rts-sim` | build-radius/occupied rejection, low-power pause/recovery, supply cap, repair and checkpoint tests |
| stance, patrol, Stop and veterancy | `trnm-rts-protocol` + `trnm-rts-sim` | typed orders, deterministic movement, kill-rank and settlement persistence tests |
| visibility, explored fog and recon reveal | `trnm-rts-sim` + `trnm-first-contact` | hidden target rejection, deterministic reveal and client visibility tests |
| NPC trust, faction rank, recruitment and mentor sparring | `trnm-rpg-core` + `trnm-campaign-core` | typed relationship and durable campaign tests |
| stat allocation, build paths and titles | `trnm-rpg-core` + `trnm-campaign-core` | preview/confirm/cancel, one-time spend, reload and distinct BattleSeed tests |
| small RPG encounter combat | `trnm-rpg-core` + `trnm-campaign-core` | attack/defend/item/withdraw and injury/loot/route consequence tests |
| active enemy combat, reinforcements and objective phases | `trnm-rts-sim` | approach/contact/relay 3-5 minute, 8-12 order tests |
| abilities and RPG stat mapping | campaign/sim cores | typed mapping and ability-rush route tests |
| save/restart/settlement | `trnm-campaign-core` | crash, recovery and duplicate-result E2E |
| room BFS, lock-aware task routing and multi-waypoint navigation | `trnm-rpg-core` + `trnm-first-contact` | unknown/locked/unreachable routes, stable next-exit and town guidance tests |
| mission objectives beyond relay capture | `trnm-campaign-core` + `trnm-rts-sim` | typed destroy/capture/defend/escort/extract definitions and Convoy Exodus E2E |
| origins, mastery challenges and conditional equipment affixes | `trnm-rpg-core` + `trnm-campaign-core` | three origins x three paths produce nine distinct seeds/stats; title requires mastery |
| reservation, opposing traffic yield and stuck recovery | `trnm-rts-sim` | typed intent/reservation, bounded replan, eight-actor uniqueness and checkpoint hash test |

Anything not listed remains historical reference only. New gameplay work must
land in the five-crate game workspace with a focused test before the matching
legacy surface can be removed from long-term storage.

The broad historical `trnm-world-domain` and `trnm-rts-core` crates now live in
this workspace. The product imports only `trnm-rpg-core` and
`trnm-rts-protocol` from the root game workspace.
