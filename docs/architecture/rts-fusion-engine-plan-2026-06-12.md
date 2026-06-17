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

The first cut has landed. The plan is now in extraction and Bevy-facing consumption mode rather than crate-bootstrap mode.

## Current Status - 2026-06-17

The following boundaries are already present on `main`:

- `trnm-rts-data` exists with `RtsMapModel`, typed First Contact Basin players, rules, actors, bounds, source manifest, and deterministic canonical hash.
- `trnm-rts-data` also owns the First Contact renderer-neutral map projection through `RtsMapRendererModel` and `first_contact_map_renderer_model(&RtsMapModel)`. It now owns the First Contact preview actor projection through `RtsFirstContactPreviewActor`, `RtsFirstContactPreviewActorKind`, and `first_contact_preview_actors(&RtsMapModel)`. Bevy consumes those projections for terrain/resource/base/objective/spawn/minimap evidence and OpenRA-like preview actors instead of keeping Bevy-local actor tables or rule-kind adapters.
- `trnm-rts-bevy-runtime` exists and owns deterministic camera, minimap, projection, path-preview, tile-line, hit-test, and runtime layout calculations.
- `trnm-rts-evidence` exists and assembles deterministic Bevy runtime adapter evidence before `trnm-world-bevy` includes that proof in release-review evidence.
- `trnm-rts-online` exists as a no-socket deterministic protocol fixture for authority resolution, visibility-scoped updates, loopback transport frames, bot plan, and arena lifecycle. It keeps hosted-service and public-launch claims false.
- Release-review CI is green on the local release artifact path with 377 checks, 0 failures, 128 packet artifacts, and a current total of about 187 seconds. The current dominant local slow checks are live-window screenshot evidence, packet semantic fixtures, and packet integrity.
- `public_launch_ready=false` and `android_s5_real_device_claimed=false` remain correct. They are blocked by real S5 device evidence, production map-pack public evidence, beta cohort evidence, commercial drill evidence, multi-node/live-traffic latency evidence, and public network exposure evidence. Public-launch blocker consistency now exposes explicit `green` and six-blocker fields, and operator handoff requires both before issuing the external-evidence handoff.

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

## Execution Slices

### Landed

The initial slice created `trnm-rts-data`:

- `RtsMapModel` for First Contact Basin.
- Typed players, rules, map actors, bounds, source manifest, and deterministic summary hash.
- Direct internal derivation from `TrillionniumRTS/mods/trnm/maps/first-contact-basin/map.yaml` and `mods/trnm/rules/trnm.yaml`.
- Tests for map size, playable bounds, players, actors, spawns, resources, objectives, expansion markers, and source manifest.

Follow-up slices have also landed enough to change the plan:

- Bevy First Contact spec/evidence consumes `trnm-rts-data`.
- First Contact terrain/resource/base/objective/spawn/minimap projection is derived in `trnm-rts-data`, with Bevy consuming `RtsMapRendererModel`.
- OpenRA-like preview map actors are derived from `first_contact_preview_actors(&first_contact_basin_map())`; spec and packet guards reject Bevy-local First Contact actor tables or rule-kind adapter regressions.
- Runtime adapter math and fixtures are split into `trnm-rts-bevy-runtime`.
- Release-review evidence assembly is split into `trnm-rts-evidence`.
- No-socket online protocol and authority fixtures are split into `trnm-rts-online`.
- Shared release-review acceptance writers are serialized with a common `flock` helper, and public-launch blocker consistency/operator handoff now expose machine-readable green/blocker-count state without claiming public launch.

### Next Local Slices

1. Move remaining renderer-neutral First Contact command-surface, runtime UI, and evidence helper code out of `trnm-world-bevy/src/lib.rs` into `trnm-rts-bevy-runtime` or `trnm-rts-evidence`, leaving `trnm-world-bevy` as the adapter/rendering owner.
2. Wire the `trnm-rts-online` authority and loopback fixtures into a Bevy-facing local multiplayer or offline bot handoff. Keep it explicitly no-socket, no-hosted-service, and no-public-launch until real network evidence exists.
3. Keep reducing packet semantic fixture and packet integrity cost only when the same semantic artifact checks remain covered. Avoid replacing semantic negatives with checksum-only assertions.
4. Treat live-window screenshot compression as a stability problem, not a simple timeout trim. Further speed work needs a better readiness signal or cheaper capture pipeline before reducing evidence frames or settle retries.
5. Keep public launch blocked until the six external evidence items are real and validator-green; templates, synthetic fixtures, local drills, and handoff manifests remain no-credit.

## Release Boundary

Internal development may use direct AGPL/GPL imports. Public distribution, external beta, hosted network service, commercial launch, or public source release require a fresh component review:

- AGPL network-source obligations for network-interactive modified programs.
- GPL distribution/source obligations for copied/derived GPL components.
- LGPL dynamic/static linking and modification obligations.
- CC BY-SA/OFL attribution/share-alike obligations for assets.

Until that review is green, keep `public_launch_ready=false` and avoid claims that imported third-party code/assets are proprietary Trillionnium-owned work.

CI and evidence boundary:

- Release-review speedups are valid only when they preserve the same artifact semantics. The current live-window evidence gate is slow by design because it captures runtime window proof; do not weaken it without a better readiness/capture signal.
- Scripts that write shared `acceptance/S6_public_launch/latest` artifacts must use the shared release-review acceptance lock or isolated temporary paths. Parallel evidence-bundle, release-packet, and integrity refreshes can create checksum drift when they bypass that discipline.
- Local green CI does not unblock public launch. The six public/S5 blockers above require real external evidence, and templates or synthetic fixtures must keep the readiness flags false.
