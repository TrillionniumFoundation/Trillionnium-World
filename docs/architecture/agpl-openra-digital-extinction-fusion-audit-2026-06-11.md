# AGPL OpenRA + Digital Extinction + Kiomet + Trillionnium Fusion Audit

Date: 2026-06-11

## Executive Verdict

The viable path is not to keep growing the current Trillionnium classic RTS shell, and not to transplant OpenRA or Digital Extinction wholesale into it. The strong path is a new copyleft RTS line with:

- OpenRA as the gameplay semantics and data-model reference: orders, deterministic frames, actor/trait composition, rules/maps/mod metadata, shroud, production, replay discipline.
- Digital Extinction as the Rust/Bevy runtime reference: 3D terrain, camera/minimap/controller separation, pathing/movement, object loading, construction, combat, energy, lobby/connector/network crates.
- Kiomet as the online Rust RTS reference: shared client/server game model, browser/WASM/WebGL delivery, visibility-scoped actor updates, lightweight binary protocol, bots, arena service boundaries, and low-friction multiplayer deployment.
- Trillionnium as the product/evidence layer: existing account/server boundaries, Bevy runner, playtest shell, release packet discipline, local evidence gates, UI/HUD polish already proven in runtime.

If AGPL is acceptable, create a separate AGPL/GPL-compatible mainline for this fused RTS. Do not silently merge GPL/AGPL code into the existing MIT Trillionnium line without changing the distribution boundary, notices, and release packaging.

## Audited Snapshots

### Trillionnium

- Path: `/home/qian/.openclaw/workspace/Trillionnium`
- Current documentation base audited: `9d543fbd9 docs: audit AGPL RTS fusion path`
- Gameplay code baseline audited: `2b146ad4f feat: map RTS viewport input through camera focus`
- License currently declared in README: MIT
- Relevant crates: `trnm-world-bevy`, `trnm-world-api`, `trnm-world-server`, `trnm-world-domain`, `trnm-world-command`, `trnm-world-ui-fragments`
- Current core issue: `trillionnium/crates/trnm-world-bevy/src/lib.rs` is about 151k lines. The RTS features are numerous, but too much logic, rendering, evidence generation, and fixtures live in one giant file.
- Current strongest asset: release/evidence discipline. The release packet already gates many RTS surfaces, and current integrity is green with public blockers preserved.

### OpenRA

- Path audited: `/tmp/openra`
- Current shallow clone audited: `387c0ea Fix location of Steam directory on GNU/Linux`, branch `bleed`, 2026-05-03
- License: GPL-3.0-or-later
- Shape: C# engine plus YAML/Lua mod data.
- Size sampled locally: about 27k lines across top-level C#/YAML/Lua count command due shallow count scope, with large module totals including `OpenRA.Mods.Common` about 158k lines and `mods` about 396k lines.
- Content sampled locally:
  - `OpenRA.Game/Network/Order.cs`
  - `OpenRA.Game/Network/OrderIO.cs`
  - `OpenRA.Game/Network/OrderManager.cs`
  - `OpenRA.Game/World.cs`
  - `OpenRA.Game/Map/Map.cs`
  - `OpenRA.Mods.Common/Orders/UnitOrderGenerator.cs`
  - `OpenRA.Mods.Common/Pathfinder/HierarchicalPathFinder.cs`
  - `OpenRA.Mods.Common/Traits/Production.cs`
- Mod-data surface sampled locally:
  - 737 YAML files under `mods`
  - 173 `map.yaml` files
  - 87 rules YAML files under mod `rules` directories

### Digital Extinction

- Path audited: `/tmp/digital-extinction-game`
- Current shallow clone audited: `5beb1fc Bump openssl from 0.10.64 to 0.10.66`, 2024-07-23
- Upstream repository status observed earlier: archived/read-only as of 2026-02-17
- License: AGPL-3.0 for source. Assets default to CC BY-SA 4.0 unless a directory-local license overrides; Fira Mono is OFL.
- Shape: 34 Rust crates, Bevy 0.13, about 34k Rust LOC, 179 test markers.
- Useful crates:
  - `controller`: mouse, selection, minimap, HUD, commands
  - `camera`: 3D RTS camera
  - `terrain` and `map`: terrain collider, map size/content/hash/io
  - `pathing`: polyanya/triangulation/path query graph
  - `movement`: altitude, kinematics, path following, repulsion, obstacles
  - `combat`: attack, health, laser, sightline, trail
  - `construction`, `objects`, `energy`
  - `multiplayer`, `net`, `connector`, `lobby`

### Kiomet

- Path audited: `/tmp/kiomet`
- Current shallow clone audited: `d3f0956 Update Readme.md`, 2025-09-09
- License: AGPL-3.0-or-later for Kiomet source.
- Shape: Rust browser RTS split into `common`, `server`, `client`, and `macros`.
- Size sampled locally: 64 Rust files, about 14.4k Rust LOC, 75 test markers.
- Game scale sampled locally:
  - `WorldChunks::SIZE = 512`, chunk size `16`, so 32x32 chunks.
  - `Ticks = kodiak_common::GenTicks<4>`, so the current Kiomet game tick is 4 Hz.
  - 27 tower types in `common/src/tower.rs`.
  - 10 unit classes in `common/src/unit.rs`.
- Useful surfaces:
  - `common/src/protocol.rs`: shared `Command` and `Update` types using `bitcode` `Encode`/`Decode`.
  - `common/src/world.rs`: shared world, actor update application, client-side tick application, path search.
  - `server/src/service.rs`: `TowerService` implementing Kodiak `ArenaService`, player joins/quits, commands, bot support, visibility-filtered updates, tick/post-update split.
  - `client/src/state.rs`: client applies server `Update` into shared `World`.
  - `client/src/game.rs`: browser game client, pan/zoom, selection/drag/path commands, WebGL rendering, Yew UI bridge.
- Important caveat: Kiomet README says current open-source build instructions are broken because downloaded Makefiles are too recent for the pinned trunk path. It also prohibits using the open-source client on official Kiomet servers because of visibility-cheating risk.

### Kodiak

- Path audited: `/tmp/kodiak`
- Tag audited: `0.1.1`, commit `c17719a`, 2025-03-17
- License sampled from SPDX headers: LGPL-3.0-or-later.
- Shape: Softbear's reusable game engine used by Kiomet. Size sampled locally: 321 Rust files, about 53.2k Rust LOC.
- Useful surfaces:
  - `server/src/service/arena_service.rs`: generic `ArenaService`, `GameRequest`, `GameUpdate`, player lifecycle, bot hooks, tick hooks.
  - `client/src/game_client.rs`: generic WASM/Yew game-client trait and update/render event hooks.
  - `common/src/actor_model/macros.rs`: actor/inbox/update macros with `Knowledge` and `Visibility` scoping.
  - `common/src/lockstep/*`: generic lockstep client/server framework with prediction, checksums, lag compensation, input windows.
  - `server/src/socket/*`: WebSocket/WebTransport abstraction, binary message encode/decode, rate limiting, keepalive.

Kodiak matters because Kiomet delegates much of the actual client/server engine and network envelope to it. If Kiomet is used as a reference, audit Kodiak at the same time; otherwise the architecture picture is incomplete.

## License Posture

AGPL acceptance changes the route, but does not erase asset and notice obligations.

- OpenRA code is GPL-3.0-or-later. Digital Extinction and Kiomet code are AGPL-3.0 family. Kodiak is LGPL-3.0-or-later. A combined distribution should be treated as a copyleft line with explicit GPL/AGPL/LGPL notices by component.
- Do not relicense OpenRA-origin files as MIT.
- Do not copy Westwood/Electronic Arts proprietary artwork, audio, videos, or original game data into Trillionnium. OpenRA contains installers and metadata for original-game content; those are not a free asset grant.
- Digital Extinction assets can be used only with CC BY-SA/OFL attribution and share-alike handling.
- Kiomet assets include branded/trademarked material, binary distribution embeds, audio credits, and AI-generated paintings. Do not reuse them blindly as Trillionnium game content; treat Kiomet primarily as an architecture reference unless a separate asset manifest is created.
- Existing MIT Trillionnium can remain as a historical product shell, but any direct OpenRA/DE/Kiomet/Kodiak code import belongs in a new AGPL/GPL/LGPL-compatible branch or package.

Recommended repository boundary:

```text
Trillionnium MIT line
  keeps current release/evidence/product history

Trillionnium RTS Fusion copyleft line
  AGPL/GPL-compatible distribution
  contains any direct OpenRA/DE/Kiomet/Kodiak-derived code or assets
```

## What Each Side Should Own

### OpenRA Owns The Rules Layer

Use OpenRA for:

- Deterministic tick/order model.
- `Order`, `OrderIO`, `OrderManager` style frame ordering.
- Actor/trait composition model.
- Mod/rules/map YAML model.
- Unit order target resolution and order priority.
- Shroud/fog semantics.
- Production queues, rally points, exits, buildability.
- Replay and sync hash discipline.
- Map importer/linter/update tool concepts.

Do not use OpenRA for:

- Rust/Bevy rendering architecture.
- Product shell/account/server integration.
- Proprietary C&C/RA/D2K content as bundled assets.

### Digital Extinction Owns The Rust/Bevy Runtime Reference

Use Digital Extinction for:

- Bevy plugin/crate separation.
- 3D camera over terrain.
- Mouse/pointer terrain raycast flow.
- Selection boxes and minimap interaction.
- Terrain collider and map bounds.
- Polyanya/pathing/triangulation ideas.
- Movement/path following/repulsion.
- Basic 3D object/GLB scene loading.
- UDP connector/lobby architecture as a transport reference.

Do not use Digital Extinction for:

- Classic RTS gameplay completeness.
- OpenRA-compatible order semantics.
- A ready-made asset pack.
- Long-term upstream support.

### Kiomet Owns The Online Rust RTS Reference

Use Kiomet and Kodiak for:

- Full-stack Rust client/server/shared-code separation.
- Browser delivery through WASM, WebGL, Yew, and Trunk.
- `common` crate pattern for shared protocol and world types.
- Binary protocol discipline with `Encode`/`Decode` request/update types.
- Visibility-scoped actor updates instead of full-world client leakage.
- Arena service boundaries: player lifecycle, bots, score, tick/post-update hooks, client data.
- Server-driven multiplayer with browser clients and low-friction deployment.
- WebSocket/WebTransport transport envelope, rate limits, keepalive, and redial concepts.
- Generic lockstep/prediction ideas from Kodiak's `common/src/lockstep`, even if Kiomet's current game does not use that as its main RTS authority model.

Do not use Kiomet for:

- Bevy or 3D runtime. It is not a DE replacement.
- Classic OpenRA/C&C-style RTS rules. It is a territory/tower RTS.
- OpenRA replay/order compatibility.
- Direct official-server client interop.
- A finished build recipe without fixing the currently broken Makefile/trunk path.
- Trusting client-side visibility; its own README warns about open-source-client visibility-cheating concerns.

### Trillionnium Owns Product, Evidence, And Current UX Proof

Keep from Trillionnium:

- Account/server/native Bevy product boundaries.
- Existing runner/service/tmux demo bring-up.
- Release-review packet and no-refresh integrity gates.
- Existing 1280x720 RTS shell, control groups, command feedback, right-click semantics, minimap, selection, visible evidence.
- Public blocker honesty: Android S5, external public launch, real beta/public evidence remain unclaimed.

Change in Trillionnium:

- Stop adding core RTS logic into the huge `trnm-world-bevy/src/lib.rs`.
- Extract deterministic RTS state into crates.
- Convert current classic shell into an adapter/client for the new simulation core.
- Change OpenRA parity gates from "analogue marker" to real compatibility gates.

## Target Architecture

Recommended new crate/package split:

```text
trillionnium/crates/trnm-rts-core
  Deterministic simulation: tick, world, actor ids, players, traits, orders, sync hash.

trillionnium/crates/trnm-rts-openra-compat
  OpenRA YAML/MiniYaml, order vocabulary, map metadata, replay/order fixtures.

trillionnium/crates/trnm-rts-data
  Trillionnium-owned rules/maps/assets, plus attribution manifests for third-party content.

trillionnium/crates/trnm-rts-bevy-runtime
  Bevy app/plugins, camera, terrain, input, rendering, audio; may draw from DE architecture.

trillionnium/crates/trnm-rts-client
  Product shell: menu, match setup, in-match HUD, local playtest flows.

trillionnium/crates/trnm-rts-net
  Frame-order lockstep, replay recorder/player, optional UDP transport.

trillionnium/crates/trnm-rts-online
  Browser/server multiplayer adapter: sessions, arena service, protocol envelope,
  visibility-filtered state updates, bots, WebSocket/WebTransport bridge.

trillionnium/crates/trnm-rts-web-client
  Optional WASM/WebGL/Yew or WebGPU web client, if browser delivery becomes a target.

trillionnium/crates/trnm-rts-evidence
  Evidence JSON/PPM helpers shared by scripts and release packet.
```

Ownership flow:

```text
OpenRA YAML / Trillionnium YAML
        -> trnm-rts-openra-compat parser
        -> trnm-rts-core deterministic state
        -> trnm-rts-bevy-runtime rendering/input adapter
        -> trnm-rts-evidence release gates
```

Runtime order flow:

```text
mouse/hotkey/minimap input
  -> OpenRA-style order targeter
  -> serialized frame order
  -> deterministic reducer
  -> world diff / render snapshot
  -> Bevy scene + HUD + evidence JSON
```

Online delivery flow:

```text
browser/native client
  -> protocol envelope
  -> trnm-rts-online arena/session
  -> OpenRA-style frame orders
  -> trnm-rts-core deterministic tick
  -> visibility-scoped client update
  -> Bevy native renderer or web renderer
```

## Integration Strategy

### Phase 0: Copyleft Line Setup

Goal: make the licensing boundary honest before code moves.

- Create a dedicated branch/worktree for the fused RTS line.
- Add top-level `COPYING`, `NOTICE`, and third-party source manifest.
- Mark the line as AGPL/GPL-compatible, not MIT-only.
- Preserve existing MIT Trillionnium history without pretending imported code remains MIT.

Acceptance gate:

- `license_fusion_manifest_gate=true`
- OpenRA/DE/Kiomet/Kodiak source commit ids recorded.
- No Westwood/EA asset bundling.

### Phase 1: OpenRA Order Compatibility Spine

Goal: replace toy command strings with real order semantics.

- Implement Rust `OpenRaOrder` equivalent with field flags for target, subject, queued, grouped actors, extra actors/location/data, target string.
- Implement serializer/deserializer fixtures compatible with OpenRA `OrderIO` concepts.
- Map current Trillionnium actions into an order vocabulary:
  - move
  - attack
  - force move
  - force attack
  - queue/shift
  - guard/follow
  - stop
  - deploy/ability
  - build/place
  - production/rally

Acceptance gate:

- Current live-input samples must emit frame orders, not only feedback chips.
- Existing UI feedback gates still pass.
- New `openra_order_frame_gate=true`.

### Phase 2: Deterministic RTS Core

Goal: move core gameplay out of the renderer.

- Create `trnm-rts-core`.
- Define stable entity ids, player ids, target ids, map cells, actor state, trait state.
- Implement tick reducer consuming frame orders.
- Implement sync hash and replay trace generation.
- Port first traits:
  - selectable
  - mobile
  - attack
  - health
  - production queue
  - rally point
  - shroud revealer
  - resource/harvest

Acceptance gate:

- Headless replay of current "queue cancel refund" and right-click samples reproduces the same final state.
- Renderer becomes a consumer of snapshots, not the source of truth.

### Phase 3: Map/Rules Data Loader

Goal: stop hardcoding the 34x34 map and unit catalog.

- Implement a MiniYaml-compatible loader for a controlled OpenRA-style subset.
- Import map metadata: map format, title, author, tileset, size, bounds, players, actors, rules.
- Support Trillionnium-owned maps first.
- Add optional OpenRA map fixture parsing for metadata only; do not bundle proprietary content.

Acceptance gate:

- At least one large Trillionnium-owned map loads from external data.
- OpenRA map metadata fixture parses without claiming asset parity.
- `large_map_coordinate_gate` is backed by loaded map data, not constants.

### Phase 4: Digital Extinction Runtime Island

Goal: prove DE can accelerate the Rust/Bevy client without derailing rules work.

- Bring a minimal DE-derived Bevy runtime island into the copyleft line:
  - terrain plane/collider
  - camera
  - pointer terrain raycast
  - selection box
  - minimap camera jump
  - path query demo
- Connect it to `trnm-rts-core` snapshots.
- Keep DE transport/network as optional until deterministic order flow is stable.

Acceptance gate:

- One Trillionnium RTS map is playable in 3D/Bevy with core orders.
- Pathing and selection are driven by core state and terrain raycast.
- Existing release packet records the DE runtime path as a distinct artifact.

### Phase 5: Production Gameplay Fusion

Goal: make the result feel like a real RTS, not an engine demo.

- OpenRA-like:
  - order priority/targeting
  - production/rally/exit
  - shroud/fog
  - build placement and power/resource constraints
  - replay and sync validation
- DE-like:
  - large terrain navigation
  - 3D units/buildings
  - energy-grid mechanics if we want the Digital Extinction flavor
- Trillionnium-like:
  - readable command feedback
  - polished HUD shell
  - evidence gates
  - product/account/session boundaries

Acceptance gate:

- 20-minute local skirmish loop with:
  - base
  - production
  - scouting/fog
  - economy or energy
  - enemy pressure
  - victory/defeat
  - replay playback
  - deterministic final sync hash

### Phase 6: Kiomet-Style Online And Browser Delivery

Goal: make the fused RTS deployable as a real online Rust game without weakening deterministic authority.

- Add `trnm-rts-online` with a narrow arena/session trait inspired by Kodiak `ArenaService`.
- Keep `trnm-rts-core` as the authority. Network layers may buffer, validate, and route frame orders, but must not become gameplay truth.
- Add a binary request/update protocol with:
  - player join/quit
  - viewport subscription
  - command/frame-order submission
  - visibility-scoped snapshot/delta update
  - ping/keepalive/redial
  - bot command injection
- Add a browser client spike only after native core replay and Bevy runtime gates are stable.
- Make client visibility cheating explicit in the threat model; do not connect modified open clients to any official production server without server-side visibility authority.

Acceptance gate:

- Two local clients can connect to one local RTS arena and submit frame orders.
- The same match can be replayed headlessly with the same final sync hash.
- Client updates are visibility-scoped and never require exposing the whole world.
- Browser delivery is documented as optional, not a blocker for the native Bevy runtime.

## OpenRA Feature Import Priority

1. Order serialization and frame manager.
2. Actor/trait vocabulary.
3. Unit order targeters and priorities.
4. Map YAML metadata and actor placement.
5. Production queue and rally/exits.
6. Shroud/fog visibility.
7. Sync hash and replay recorder/player.
8. Pathfinding heuristics after core map representation stabilizes.
9. Lua/scripting only after deterministic core is stable.

## Digital Extinction Feature Import Priority

1. Bevy plugin/crate structure.
2. Camera/controller/minimap input architecture.
3. Terrain collider and pointer raycast.
4. Pathing crate concepts.
5. Movement/path following/repulsion.
6. Object/GLB asset loading.
7. Combat laser/sightline as optional flavor.
8. UDP connector only after OpenRA-style order lockstep is in place.

## Kiomet/Kodiak Feature Import Priority

1. Crate split: `common` shared game/protocol types, `server` arena authority, `client` renderer/input.
2. `ArenaService`-like boundary for match lifecycle, player lifecycle, bots, score, tick/post-update.
3. Binary request/update envelope using explicit encode/decode types.
4. Visibility-scoped actor/snapshot updates based on client knowledge and viewport/subscription state.
5. Browser/WASM/WebGL delivery pattern, after native RTS core is no longer renderer-owned.
6. WebSocket/WebTransport socket abstraction, rate limiting, keepalive, and reconnect/redial.
7. Kodiak lockstep ideas: input windows, prediction, checksums, lag compensation.
8. Kiomet territory/tower gameplay ideas only as optional modes or design inspiration, not as the main OpenRA-style rules layer.

## Trillionnium Refactor Priority

1. Freeze new core RTS work in `trnm-world-bevy/src/lib.rs`.
2. Extract current command/state structs into `trnm-rts-core`.
3. Extract evidence helpers into `trnm-rts-evidence`.
4. Keep the current 1280x720 shell as a compatibility renderer.
5. Move current live-input scripts to hit the new core through the client adapter.
6. Replace "OpenRA-style analogue" gates with real imported fixture gates.

## Asset Policy

Allowed:

- Trillionnium-owned original assets.
- Digital Extinction CC BY-SA assets with attribution and share-alike handling in the copyleft line.
- OpenRA engine/rules/code concepts under GPL-compatible notices.
- OpenRA installer/metadata references as references, if notices are preserved.

Not allowed as bundled game content without separate rights:

- Westwood/EA proprietary art, audio, videos, MIX content, original game data.
- Asset claims like "OpenRA/Westwood asset parity" unless actual rights and fixtures prove the claim.

## Main Risks

- License drift: accidentally mixing GPL/AGPL code into the MIT line.
- Monolith drift: continuing to add features to the 151k-line Bevy file instead of extracting core.
- False parity: claiming OpenRA compatibility from superficial UI screenshots.
- Asset contamination: treating OpenRA content installer metadata as a license to bundle original assets.
- Determinism gap: Bevy runtime state driving gameplay directly instead of consuming deterministic core snapshots.
- Online-authority gap: using Kiomet-style browser delivery before server-side visibility and replay authority are in place.
- Build-recipe gap: Kiomet's current open-source build instructions are not cleanly reproducible without Makefile/trunk fixes.
- Scope explosion: trying to import OpenRA, DE, multiplayer, 3D rendering, and all gameplay at once.

## Recommended First Week

1. Create copyleft fusion branch/worktree and license manifest.
2. Add `trnm-rts-core` with deterministic order/state skeleton.
3. Add `trnm-rts-openra-compat` with Rust order serializer fixtures.
4. Make the current Trillionnium live-input path emit real frame orders.
5. Add a new release gate: current samples must replay through headless core before rendering.
6. Add a minimal data-loaded map fixture replacing the hardcoded 34x34 constants.
7. Spike DE camera/terrain/raycast in a separate proof command, not in the production runner yet.
8. Add a Kiomet/Kodiak audit fixture that proves the planned `trnm-rts-online` boundary is source-backed but deferred until deterministic core gates are green.

## Revised Fusion Verdict

The four-source split should be:

- OpenRA defines what a classic RTS match means.
- Digital Extinction shows how a Rust/Bevy 3D RTS can be structured and rendered.
- Kiomet/Kodiak shows how a real Rust browser multiplayer game packages shared logic, server authority, bots, deltas, and deployment.
- Trillionnium provides the product shell, native runner, and evidence gates.

Kiomet should not replace Digital Extinction. Kiomet should replace hand-wavy multiplayer plans.

## One-Sentence Direction

Build a new copyleft Trillionnium RTS line where OpenRA defines what an RTS match means, Digital Extinction shows how a Rust/Bevy RTS can run, Kiomet/Kodiak shows how Rust multiplayer can ship online, and Trillionnium keeps the product shell plus evidence discipline that prevents fake completion claims.
