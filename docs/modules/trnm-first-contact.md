---
status: current-candidate
owner: Trillionnium World native client and player experience
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-first-contact technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A Bevy native client for the RPG-to-RTS-to-debrief-to-town loop. It wires asset/audio loading, campaign flow/UI, HUD, map loading, online compatibility transport, command journal, renderer, simulation adapter, frame timing, settings/save slots and evidence adapters.

## Responsibilities and non-responsibilities

Owns presentation, input and local orchestration. It is untrusted for canonical online state, identity, completion, wallet balance, settlement success and finality.

## Public interfaces and data contracts

`FirstContactLivePlugin` is the runtime composition entry. Input is normalized into typed intents/orders. Offline mode calls campaign/simulation APIs; attached mode submits through bounded transports and consumes verified snapshots/receipts. Files use explicit schemas and version fields.

## State model and invariants

Local settings, saves, command journal, replay, map/atlas assets and UI mode are client-owned. Offline campaign mutation occurs only through `trnm-campaign-core`; simulation mutation only through validated RTS orders. Online mode disables local authority simulation. Displayed authority/economy state retains source and verification status.

## Dependency direction

Bevy owns presentation scheduling; Rodio owns local audio; HTTP/WebSocket libraries are transport only. Domain semantics stay in RPG/campaign/RTS/economy/online crates. Network or disk work must not block the render/update thread. Cross-thread results are typed and bounded.

## Concurrency and cancellation

The Bevy main thread owns UI/render state. Dedicated workers own blocking HTTP, WebSocket and reconciliation. Channels have fixed capacities and explicit overflow behavior. Command attempts are journaled before enqueue. Cancellation or disconnect cannot publish a fabricated success; late results are generation/session checked.

## Durability and recovery

Save/settings/journal writes validate parent ownership, reject unsafe links where supported, write bounded temporary files, flush and atomically replace. A command journal records exact bytes/identity before transport and removes entries only after verified receipt. Corrupt slots are isolated; recovery is explicit.

## Failure, retry and idempotency

Missing/corrupt assets, unsupported saves, crossed protocols, invalid snapshots, journal conflicts and uncertain economy results fail closed with actionable player state. Exact duplicates may retry; altered duplicates reject. Offline fallback is explicit and never represented as canonical online operation.

## Versioning and migration

Changing system order, input semantics, save/replay interpretation, asset schema, online protocol or presentation of authority/economy state requires fixtures and migration notes. Supported OS/render/audio/input combinations are release-candidate metadata, not inferred from source compilation.

## Resource budgets and performance

Frame/update work, network polling, channel drain, map entities, UI nodes, textures/audio, save/replay bytes and reconnect suffixes are bounded. Frame evidence records 30/60 FPS targets plus input/network stalls on named hardware. Background work has deadlines and cancellation.

## Observability and security

Local telemetry is privacy-minimized and opt-in where user data is involved. Engineering logs record mode, component versions, state/cursor digests, frame stalls and stable errors, not tokens or private chat/value material. Security covers journal tampering, asset traversal, forged snapshots and authority confusion.

## Verification traceability

Tests cover plugin/system ordering, assets/maps, campaign closed loop, snapshot/reconnect, journal crash recovery, replay, input profiles, frame budgets, packaging/install and migration. Independent human sessions cover comprehension, keyboard/mouse, high contrast, subtitles, low motion and viewport/device matrices.

## Known gaps and change checklist

Split campaign flow/UI/online/render/settings into true modules; publish a Bevy schedule and thread/channel diagram; finish multi-OS signing/update/rollback; retain performance baselines; and conduct consented human/accessibility validation on exact builds. Automated screenshots cannot close human evidence.
