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

## Current Status - 2026-06-18

The following boundaries are already present on `main`:

- `trnm-rts-data` exists with `RtsMapModel`, typed First Contact Basin players, rules, actors, bounds, source manifest, and deterministic canonical hash.
- `trnm-rts-data` also owns the First Contact renderer-neutral map projection through `RtsMapRendererModel` and `first_contact_map_renderer_model(&RtsMapModel)`. It now owns the First Contact preview actor projection through `RtsFirstContactPreviewActor`, `RtsFirstContactPreviewActorKind`, and `first_contact_preview_actors(&RtsMapModel)`. Bevy consumes those projections for terrain/resource/base/objective/spawn/minimap evidence and OpenRA-like preview actors instead of keeping Bevy-local actor tables or rule-kind adapters.
- `trnm-rts-bevy-runtime` exists and owns deterministic camera, minimap, projection, path-preview, tile-line, hit-test, runtime layout calculations, the First Contact offline-adapter runtime application contract, and the First Contact offline-adapter player-screen consumption review contract.
- `trnm-rts-evidence` exists and assembles deterministic Bevy runtime adapter evidence before `trnm-world-bevy` includes that proof in release-review evidence.
- `trnm-rts-online` exists as a no-socket deterministic protocol fixture for authority resolution, visibility-scoped updates, loopback transport frames, bot plan, arena lifecycle, a Bevy-facing local handoff summary, and an offline loopback adapter summary for local multiplayer/bot handoff. The offline adapter now also carries dedicated local UI/action replay, runtime handoff, and Bevy-runtime review-input builders derived from retained/pruned control-group replay fixtures and server-authoritative accepted/rejected fixture orders. It keeps client-prediction, rollback-netcode, hosted-service, socket, and public-launch claims false.
- The First Contact Bevy spec now consumes the no-socket offline adapter into local `NativeFirstPlayableRuntime` action-replay state through the `trnm-rts-online` review-input builder and `trnm_rts_online_offline_adapter_runtime_handoff_v1`, translates that handoff through the Bevy-free `trnm_rts_bevy_runtime_first_contact_offline_adapter_runtime_application_v1` contract, then hands the resulting player-screen snapshot to `trnm_rts_bevy_runtime_first_contact_offline_adapter_consumption_v1` for Bevy-free review: the server-authoritative accepted move reaches the runtime command queue/stamp and the fogged attack rejection is suppressed from UI state. This is still local/offline proof only, not network readiness.
- The same First Contact proof now extends that offline adapter consumption into a visible player-screen/session handoff surface: room/map, camera, visibility, queues, supply, objective state, and route overlay remain coherent while the accepted server move replaces the local command surface. This is still explicitly local, no-socket, no-hosted-service, no-client-prediction, no-rollback, and no-public-launch evidence.
- Release-review CI is green on the local release artifact path with 377 checks, 0 failures, 128 packet artifacts, and a current total of about 193 seconds. The current dominant local slow checks are live-window screenshot evidence, packet semantic fixtures, and packet integrity. Live-window screenshot evidence now emits capture/readiness diagnostics so future speed work can distinguish window readiness, frame capture retries, and settle/key retries before trimming any proof.
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
- No-socket online protocol and authority fixtures are split into `trnm-rts-online`, with `RtsOnlineLocalHandoff` and `RtsOnlineOfflineAdapterSummary` exposing the local Bevy-facing handoff/adapter contracts while keeping socket/hosted-service/public-launch flags false.
- Bevy First Contact spec consumes the offline adapter into a local action-replay summary, proving accepted server orders mutate the Bevy runtime command state while rejected fogged commands stay out of the UI/action queue. The upstream offline adapter contract now exposes accepted/blocked control-group replay inputs, bounded history semantics, server-authoritative runtime handoff labels/stamp, and runtime review-input builders before Bevy renders them; `trnm-rts-bevy-runtime` translates those labels/stamps into the runtime application consumed by `NativeFirstPlayableRuntime`, then owns the player-screen/session handoff review gates.
- Shared release-review acceptance writers are serialized with a common `flock` helper, release-review CI exposes explicit check-count aliases, live-window screenshot evidence exposes capture/readiness diagnostics, and public-launch blocker consistency/operator handoff now expose machine-readable green/blocker-count state without claiming public launch.

### Next Local Slices

1. Move remaining renderer-neutral First Contact command-surface, runtime UI, and evidence helper code out of `trnm-world-bevy/src/lib.rs` into `trnm-rts-bevy-runtime`, `trnm-rts-online`, or `trnm-rts-evidence`, leaving `trnm-world-bevy` as the adapter/rendering owner. The offline-adapter runtime application, online-owned review-input mapping, and player-screen consumption review have moved; continue with command-surface and broader session-transition helpers.
2. Continue extending the Bevy-facing offline adapter path beyond the runtime application/player-screen handoff review into broader local session/UI transitions and additional review surfaces. The next slice should consume more screen transitions from the `trnm_rts_online_offline_adapter_runtime_handoff_v1` source and review them through Bevy-free runtime/evidence contracts rather than re-deriving fixture semantics inside `trnm-world-bevy`, while keeping it explicitly no-socket, no-hosted-service, no-client-prediction, no-rollback-netcode, and no-public-launch until real network evidence exists.
3. Keep reducing packet semantic fixture and packet integrity cost only when the same semantic artifact checks remain covered. Avoid replacing semantic negatives with checksum-only assertions.
4. Use the new live-window capture/readiness diagnostics to identify whether the 16s path is window startup, xwd/ffmpeg capture, frame-change settling, or repeated key attempts. Do not reduce frames or settle retries until the diagnostics show a cheaper path preserves the same evidence.
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
