# RTS Third-Party Source Manifest

Date: 2026-06-11

This manifest tracks third-party RTS sources audited for the Trillionnium RTS fusion route. As of 2026-06-12, the project allows direct internal AGPL/GPL imports for local-only acceleration, but every copied or derived component must stay recorded here with a release constraint. No Westwood, EA, Warcraft III, or other proprietary original-game source/assets may be copied into the Trillionnium tree.

## Project-Owned License Posture

- Trillionnium-owned code: internal/proprietary; no public open-source license grant.
- Public release status: not claimed.
- Source release status: not claimed.
- Third-party imports: must be recorded here before copied or derived code/assets land.
- Internal direct imports: allowed for AGPL/GPL/LGPL components while the project is not public and not externally served, but each import must keep source/license/replacement tracking.

## Sources

### OpenRA

- Upstream: `https://github.com/OpenRA/OpenRA`
- Local audit path: `/tmp/openra`
- Audited commit: `387c0ea Fix location of Steam directory on GNU/Linux`
- Audited branch: `bleed`
- License: GPL-3.0-or-later
- Current Trillionnium use: reference-only
- Copied into Trillionnium: no
- Intended use: order/frame semantics, actor/trait model, rules/map metadata, replay/sync discipline, production/shroud/pathing concepts.
- Explicit exclusions: Westwood/Electronic Arts artwork, audio, videos, MIX content, original game data, and installer-fetched proprietary content.

### Digital Extinction

- Upstream: `https://github.com/DigitalExtinction/Game`
- Local audit path: `/tmp/digital-extinction-game`
- Audited commit: `5beb1fc Bump openssl from 0.10.64 to 0.10.66`
- Repository status observed during audit: archived/read-only as of 2026-02-17
- License: AGPL-3.0 for source
- Asset licenses sampled: CC BY-SA 4.0 by default unless directory-local license overrides; Fira Mono is OFL.
- Current Trillionnium use: reference plus approved direct-internal-import candidate
- Copied into Trillionnium: no
- Intended use: Rust/Bevy RTS runtime architecture, camera/controller/minimap separation, terrain/pathing/movement/combat/construction/lobby/network references.
- Direct import lane: terrain/map/pathing/movement/camera/controller/minimap modules may be copied or modified into isolated AGPL-marked crates/paths after file-level entries are added to this manifest.
- Explicit exclusions: untracked asset reuse, blind crate transplant into non-isolated Trillionnium-owned code, or treating DE as a replacement game.

### Kiomet

- Upstream: `https://github.com/SoftbearStudios/kiomet`
- Local audit path: `/tmp/kiomet`
- Audited commit: `d3f0956 Update Readme.md`
- License: AGPL-3.0-or-later
- Current Trillionnium use: reference plus approved direct-internal-import candidate
- Copied into Trillionnium: no
- Intended use: shared client/server model, protocol/update envelope concepts, browser/WASM/WebGL delivery, bots, visibility-scoped server updates, arena lifecycle.
- Direct import lane: common/protocol/world visibility and server arena concepts may be copied or modified into isolated AGPL-marked crates/paths after file-level entries are added to this manifest.
- Explicit exclusions: Kiomet branding, audio, generated paintings, production server compatibility, and client-trusted gameplay authority.

### Kodiak

- Upstream: `https://github.com/SoftbearStudios/kodiak`
- Local audit path: `/tmp/kodiak`
- Audited tag: `0.1.1`
- Audited commit: `c17719a`
- License sampled from SPDX headers: LGPL-3.0-or-later
- Current Trillionnium use: reference-only
- Copied into Trillionnium: no
- Intended use: arena service boundaries, binary socket envelopes, WebSocket/WebTransport abstraction, actor visibility macros, lockstep/prediction/checksum framework concepts.
- Explicit exclusions: blind engine dependency before deterministic core gates are green.

### TrillionniumRTS

- Upstream: `local:/home/qian/.openclaw/workspace/TrillionniumRTS`
- Local audit path: `/home/qian/.openclaw/workspace/TrillionniumRTS`
- Audited commit: `6fd679b576a1130558cd69b4e3ab2817f819dd22`
- License: GPL-3.0-or-later OpenRA Mod SDK prototype boundary
- Current Trillionnium use: direct internal GPL-derived data seed
- Copied or derived into Trillionnium: yes, typed map/rules data in `trillionnium/crates/trnm-rts-data`
- Source paths:
  - `mods/trnm/maps/first-contact-basin/map.yaml`
  - `mods/trnm/rules/trnm.yaml`
  - `mods/trnm/rules/mpspawn.yaml`
  - `mods/trnm/tilesets/trnm.yaml`
  - `mods/trnm/sequences/trnm.yaml`
- Intended use: First Contact Basin map/rules/content seed for the Bevy-free data model and later player-screen renderer.
- Release constraint: internal-only until GPL component review or clean replacement.
- Explicit exclusions: OpenRA engine code, Westwood/Electronic Arts original content, and proprietary third-party assets.

## Import Gate

Before any third-party RTS code or asset becomes copied or derived in Trillionnium, add:

- Upstream repository URL and commit/tag.
- Exact upstream file or asset path.
- Exact local destination path.
- License and required notices.
- Whether the local work is copied, translated, derived, or clean-room reference.
- Release constraint: internal-only, source-offer required, attribution required, share-alike required, or prohibited.
- Evidence that no proprietary Westwood/EA original game content is bundled.

## Current Copied/Derived Entries

### `trnm-rts-data` First Contact Basin

- Local destination: `trillionnium/crates/trnm-rts-data`
- Source: TrillionniumRTS entries listed above.
- Copied/derived status: derived typed Rust data model from local OpenRA Mod SDK YAML.
- Integration mode: `gpl_internal_component`
- Release constraint: internal-only until GPL component review or clean replacement.
- Current scope: map metadata, players, map actors, rule summaries, source manifest, deterministic summary hash.
- Runtime status: Bevy-free data boundary landed; Bevy player screen still needs a follow-up adapter slice before it consumes this crate directly.
