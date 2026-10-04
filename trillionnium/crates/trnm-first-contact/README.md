# `trnm-first-contact`

Status: **pre-alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Provides the native Bevy player client, authored-map presentation, input/UI/audio, offline campaign shell, and online-authority adapter.

## Responsibilities

- Player input, selection, HUD, campaign UI, renderer, audio, and map/atlas lifecycle.
- Offline deterministic simulation adapter and online snapshot/command adapter.
- Client command journal and frame/input/network timing evidence.
- Emitting versioned orders/results through the campaign authority.

It does not mutate persistent progression outside `trnm-campaign-core`, become authoritative online, or turn automated traversal/render smoke into human-usability evidence.

## Invariants

- Online mode disables local authority simulation.
- Map transitions retire scoped entities before loading the next authored map.
- Credentials never enter the command journal.
- Settings and save slots remain isolated.
- Settlement passes through campaign authority.
- Automated rendering evidence remains distinct from real-human evidence.

Missing/corrupt assets, invalid saves, journal corruption, snapshot/hash regression, or authority mismatch must quarantine or fail closed. A disconnected stream may use bounded recovery, never local authority takeover.

## Run and test

```bash
cargo run --manifest-path trillionnium/Cargo.toml -p trnm-first-contact
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-first-contact --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-first-contact --all-targets --locked -- -D warnings
```

`TRNM_ASSET_ROOT` may override the asset root. `TRNM_WORLD_BEVY_LOW_SPEC=1` selects low-spec offline window behavior. Do not commit real online credentials.

## Compatibility and security

A client release declares supported campaign, RTS, online protocol/build, asset manifest, and save versions. Compatibility probes remain required while legacy pairs are advertised. Protect sessions, local saves/journals, input ownership, asset paths, and URL handling; never log credentials.

## References

- [`trillionnium-rpg-rts-closed-loop-v1.md`](../../../docs/development/trillionnium-rpg-rts-closed-loop-v1.md)
- [`trnm-online-authority-v3.md`](../../../docs/development/trnm-online-authority-v3.md)
- [`trnm-native-game-release-gates-v1.md`](../../../docs/development/trnm-native-game-release-gates-v1.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)

## Detailed design

- [`trnm-first-contact detailed design`](../../../docs/modules/trnm-first-contact-design.md)
