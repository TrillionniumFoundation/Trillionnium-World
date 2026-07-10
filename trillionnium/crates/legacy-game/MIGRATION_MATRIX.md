# Legacy Gameplay Migration Matrix

The legacy monolith is archived and feature-gated. Product mechanics were
reimplemented as focused, Bevy-free behavior with product-owned tests; no old
map data or whole-world state was reconnected.

| Legacy behavior family | Product owner | Product regression |
| --- | --- | --- |
| tile pathing and blocked terrain | `trnm-rts-sim` | authored 40x24 map route test |
| unit occupancy and map-aware pursuit | `trnm-rts-sim` | deterministic checkpoint and one-order rejection tests |
| resource collection/economy | `trnm-rts-sim` + `trnm-campaign-core` | victory resource settlement / withdrawal zero-delta E2E |
| target and control-group commands | `trnm-rts-core` + `trnm-first-contact` | exact consumed-order adapter test |
| active enemy combat and objective phases | `trnm-rts-sim` | approach/contact/relay 3-5 minute test |
| abilities and RPG stat mapping | campaign/sim cores | typed mapping and ability-rush route tests |
| save/restart/settlement | `trnm-campaign-core` | crash, recovery and duplicate-result E2E |

Anything not listed remains historical reference only. New gameplay work must
land in the five-crate game workspace with a focused test before the matching
legacy surface can be removed from long-term storage.
