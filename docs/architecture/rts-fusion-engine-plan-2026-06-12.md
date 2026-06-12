# RTS Fusion Engine Plan

Date: 2026-06-12

This plan turns the current heavy `trnm-world-bevy` RTS implementation into module boundaries that can absorb direct internal AGPL/GPL imports where that is faster, while keeping a future replacement path for public release or hosted service.

## Direction

Do not start a separate rewrite. Keep the current playable Bevy RTS on `main`, then cut the heaviest surfaces into import-friendly crates:

- `trnm-rts-core`: deterministic world, actors, orders, fog, occupancy, resources, replay hashes.
- `trnm-rts-data`: maps, rules, tiles, spawns, asset manifests, OpenRA/TrillionniumRTS-style YAML-derived data.
- `trnm-rts-bevy-runtime`: camera, input, minimap, terrain renderer, movement/path preview, Bevy adapter.
- `trnm-rts-online`: shared world/protocol/update envelope, arena lifecycle, visibility-scoped updates, bots.
- `trnm-rts-evidence`: screenshots, JSON gates, PPM helpers, release-review proof generation.

The first cut lands `trnm-rts-data` so First Contact Basin map/rules stop living only as hard-coded Bevy constants.

## Reference Ownership

### OpenRA

- Role: rules semantics, actor/trait model, order/frame discipline, YAML map/rules shape, fog, production, replay/sync.
- Import policy: mostly clean-room semantics; direct GPL snippets require explicit internal component tracking.
- Main reason: OpenRA is the best rules vocabulary reference, not the best Rust runtime.

### Digital Extinction

- Role: Bevy runtime structure: terrain, map, pathing, movement, camera, controller, minimap, construction, combat, lobby/network crate boundaries.
- Import policy: direct internal AGPL import is allowed when it materially speeds the work. Imported code must be isolated under an AGPL-marked crate/path and recorded in the source manifest.
- Main reason: it is the closest Rust/Bevy RTS runtime reference.

### Kiomet

- Role: large-world/shared-state architecture: common world/protocol, chunked world, visibility-scoped updates, server authority, bots.
- Import policy: direct internal AGPL import is allowed for protocol/world/visibility ideas when isolated; rendering/client assumptions must not become gameplay authority.
- Main reason: it is the most complete strict Rust online RTS reference, but not a Bevy/3D renderer.

### Kodiak

- Role: arena service boundary, binary socket envelope, WebSocket/WebTransport, lockstep/prediction/checksum framework.
- Import policy: LGPL component tracking before direct dependency or copied code.
- Main reason: Kiomet's architecture is incomplete without Kodiak.

### TrillionniumRTS

- Role: local content seed: First Contact Basin `map.yaml`, Trillionnium rules, placeholder art slots.
- Import policy: direct internal GPL-derived data is allowed and tracked because this OpenRA Mod SDK prototype is local and already isolated.
- Main reason: it gives an immediate map/rules data source for the Bevy player screen.

## Import Lanes

### Lane A: Direct Internal Import

Use this for AGPL/GPL/LGPL modules that can accelerate local development before public release.

Requirements:

- Record upstream/local path, commit, license, copied/derived status, local destination, and release constraint.
- Keep imports behind Trillionnium traits/interfaces so they can be replaced.
- Do not bundle Westwood, EA, Warcraft III, or other proprietary game data.
- Mark public launch and hosted-service readiness false until license obligations are reviewed.

### Lane B: Clean-Room Semantics

Use this for OpenRA rules/order concepts and any module whose direct import would bind too much of the mainline.

Requirements:

- Document upstream file/behavior read.
- Implement Trillionnium-owned types and tests.
- Keep compatibility evidence honest: no OpenRA binary replay/runtime parity claim unless proven.

### Lane C: Project-Owned Data

Use this for Trillionnium-owned rules, maps, and art generated specifically for this project.

Requirements:

- Keep source paths and generation provenance.
- Add asset manifests before bundling new art.
- Prefer typed Rust data models over ad hoc strings.

## First Execution Slice

The current slice creates `trnm-rts-data`:

- `RtsMapModel` for First Contact Basin.
- Typed players, rules, map actors, bounds, source manifest, and deterministic summary hash.
- Direct internal derivation from `TrillionniumRTS/mods/trnm/maps/first-contact-basin/map.yaml` and `mods/trnm/rules/trnm.yaml`.
- Tests for map size, playable bounds, players, actors, spawns, resources, objectives, expansion markers, and source manifest.

Next slices:

1. Make the Bevy First Contact spec evidence consume `trnm-rts-data` instead of local constants.
2. Make the player-screen map renderer consume `RtsMapModel` actors/rules for terrain/resource/base/minimap projection.
3. Split runtime adapter surfaces using the Digital Extinction lane: camera/minimap/pathing first.
4. Add `trnm-rts-online` protocol sketches using Kiomet/Kodiak lane: chunk visibility, update envelope, bots, arena lifecycle.

## Release Boundary

Internal development may use direct AGPL/GPL imports. Public distribution, external beta, hosted network service, commercial launch, or public source release require a fresh component review:

- AGPL network-source obligations for network-interactive modified programs.
- GPL distribution/source obligations for copied/derived GPL components.
- LGPL dynamic/static linking and modification obligations.
- CC BY-SA/OFL attribution/share-alike obligations for assets.

Until that review is green, keep `public_launch_ready=false` and avoid claims that imported third-party code/assets are proprietary Trillionnium-owned work.
