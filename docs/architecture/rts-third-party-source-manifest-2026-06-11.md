# RTS Third-Party Source Manifest

Date: 2026-06-11

This manifest tracks third-party RTS sources audited for the Trillionnium RTS fusion route. As of this file's creation, the entries below are reference-only: no OpenRA, Digital Extinction, Kiomet, Kodiak, Westwood, EA, or other third-party RTS source/assets are copied into the Trillionnium tree by this manifest.

## Project-Owned License Posture

- Trillionnium-owned code: internal/proprietary; no public open-source license grant.
- Public release status: not claimed.
- Source release status: not claimed.
- Third-party imports: must be recorded here before copied or derived code/assets land.

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
- Current Trillionnium use: reference-only
- Copied into Trillionnium: no
- Intended use: Rust/Bevy RTS runtime architecture, camera/controller/minimap separation, terrain/pathing/movement/combat/construction/lobby/network references.
- Explicit exclusions: untracked asset reuse, blind crate transplant, or treating DE as a replacement game.

### Kiomet

- Upstream: `https://github.com/SoftbearStudios/kiomet`
- Local audit path: `/tmp/kiomet`
- Audited commit: `d3f0956 Update Readme.md`
- License: AGPL-3.0-or-later
- Current Trillionnium use: reference-only
- Copied into Trillionnium: no
- Intended use: shared client/server model, protocol/update envelope concepts, browser/WASM/WebGL delivery, bots, visibility-scoped server updates, arena lifecycle.
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

## Import Gate

Before any third-party RTS code or asset becomes copied or derived in Trillionnium, add:

- Upstream repository URL and commit/tag.
- Exact upstream file or asset path.
- Exact local destination path.
- License and required notices.
- Whether the local work is copied, translated, derived, or clean-room reference.
- Release constraint: internal-only, source-offer required, attribution required, share-alike required, or prohibited.
- Evidence that no proprietary Westwood/EA original game content is bundled.
