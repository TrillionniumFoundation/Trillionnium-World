# Frozen Legacy Final Index — 2026-07-11

Status: final historical and recovery index after removing the compiled legacy
workspace from the current checkout.

## Recovery anchors

- Pre-removal product head: `d44d8930c917b55da7b23eb19e9645feb8f4ee59`.
- Last legacy source/Cargo reorganization: `99b0c655b714d5e3767ea2d4b21e59eafba4402e`.
- Initial five-crate isolation: `0103ff738898b949b1632f2798b8380219db7b15`.
- No Git history was rewritten during removal.

To inspect or recover the exact final legacy tree without reconnecting it to the
product:

```bash
git ls-tree -r d44d8930c917b55da7b23eb19e9645feb8f4ee59 -- trillionnium/crates/legacy-game
git show d44d8930c917b55da7b23eb19e9645feb8f4ee59:trillionnium/crates/legacy-game/MIGRATION_MATRIX.md
git worktree add /tmp/trnm-frozen-legacy d44d8930c917b55da7b23eb19e9645feb8f4ee59
```

## Final inventory

- 14 crates, 80 tracked files and 233,552 Rust source lines.
- `trnm-world-bevy/src/legacy.rs`: 167,012 lines.
- Tracked bytes under the removed subtree: 9,492,395; final working-tree size
  including ignored build residue was approximately 16 MiB.
- The product depended on zero legacy crates. The only cross-boundary edge was
  the reverse dependency `trnm-world-bevy -> trnm-first-contact`.

Removed crates:

- `trnm-world-domain`
- `trnm-rts-core`
- `trnm-world-command`
- `trnm-rts-data`
- `trnm-rts-bevy-runtime`
- `trnm-rts-evidence`
- `trnm-rts-online`
- `trnm-world-projection`
- `trnm-world-map-provider`
- `trnm-world-ui-fragments`
- `trnm-world-api`
- `trnm-world-server`
- `trnm-world-bevy`
- `trnm-world-dev-env`

## Final test baseline

The final serial run used:

```bash
cargo test --workspace --no-fail-fast -- --test-threads=1
```

Result: 357/360 passed. The three stable failures were historical visual
evidence assertions, not current product gates:

1. `classic_first_contact_atlas_readability_guard_tracks_project_owned_frames`
2. `classic_first_contact_marker_budget_guard_mutes_noninteractive_gallery`
3. `classic_first_contact_player_atlas_objective_marker_draws_micro_gold_pips`

The intermittently parallel-polluted title-menu fixture passed in the serial
run. The current five-crate product owns its own deterministic unit, E2E,
Clippy, release-build and live-window gates.

## Migration disposition

The archive's useful behavior families were reimplemented clean-room in the
five product crates: world routing, typed missions, growth/mastery, logistics
economy, supply/power/building rules, command/control groups, fog/recon,
production lifecycle, stance/patrol/veterancy, deterministic traffic recovery,
save slots/pause/settings, quest chains, readiness, adaptive AI, journal,
identity confirmation, difficulty and four original campaign maps.

No source line remained with positive direct-migration value. Small helpers are
cheaper to rewrite than to provenance-audit; large modules use obsolete
World/CEX/evidence authorities and would reintroduce dual ownership.

`trnm-rts-data` explicitly recorded a GPL-3.0-or-later OpenRA Mod SDK prototype
boundary with `copied_or_derived=true` and
`internal_only_until_gpl_component_review_or_replacement`. That code and data
must not be restored into the product.

## CEX adapter disposition

The historical CEX implementation remains in the separate CEX repository at
`services/consumer-entry-api/src/trillionnium_world_adapters.rs` (744 lines,
last changed by CEX commit `a21f161091195e3ea4b7da8f670d7302926db180`).
It implements six old World adapter roles but points at removed root paths for
`trnm-world-api`, `trnm-world-domain` and `trnm-world-projection`; CEX
`cargo metadata` fails on those missing manifests.

The cached evidence from 2026-06-29 that was revalidated on 2026-07-10 is not a
live build proof and grants no current TRNM credit. The old readiness gate is
retired/blocked. Any future integration must define a new narrow contract
outside the five game crates around current save/campaign/battle-result
authorities; it must not reconnect this historical workspace.
