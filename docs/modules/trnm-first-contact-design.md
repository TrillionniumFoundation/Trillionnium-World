---
status: current-candidate
owner: trillionnium-world-client
module: trnm-first-contact
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-first-contact detailed design

## Scope and authority

`trnm-first-contact` is the native Bevy client for the RPG-to-RTS-to-debrief-to-town product loop. It owns presentation, input, local campaign shell/UI, authored map/atlas/audio loading, deterministic simulation adaptation, local settings/save slots, replay inspection, compatibility transport, command-attempt journaling, and player-facing status/evidence adapters. It is untrusted for canonical online state, participant identity, completion, wallet balance, settlement success, and finality.

## Public interfaces

`FirstContactLivePlugin` composes asset/audio loading, campaign flow/UI, HUD, map loading, online compatibility, command journal, renderer, simulation adapter, frame timing, and view math. External inputs are keyboard/mouse/control profiles, local save/settings/replay/assets, admitted server snapshots/receipts, and typed player intents. Outputs are rendered/audio state, typed campaign/RTS/network commands, bounded local journals, verified views, and engineering evidence. Presentation never becomes a domain or online authority API.

## State model and invariants

The client distinguishes offline-local, compatibility-attached, reconnect/resync, pending settlement, debrief, replay, and failure states. Offline campaign mutation occurs only through `trnm-campaign-core`; local battle mutation only through validated `trnm-rts-protocol` and `trnm-rts-sim`. Attached mode consumes verified server state and disables local authority simulation. UI labels preserve the difference between local value, pending/verified settlement, compatibility laboratory, and Nakama-canonical public online. Map transitions remove prior scoped entities before rebuilding.

## Dependency direction

Bevy and Rodio are presentation/runtime adapters. Domain semantics stay in RPG/campaign/RTS/economy/online protocol crates. Asset/network/filesystem adapters may translate into typed events but cannot mutate domain internals directly. The client must not import CEX/Nakama private implementation or credentials, sign canonical completion, or use sibling repository paths. Renderer/HUD observe view models; they do not parse display strings back into IDs, commands, balances, or authority state.

## Execution, concurrency and cancellation

The correctness-relevant system chain is input to typed intent, adapter submission, deterministic state/application, settlement staging, then UI/render/audio observation. Network, reconciliation, package I/O, and other potentially blocking work must not run on the Bevy update/render thread. Bounded worker queues return typed events with generation/revision identity. Cancellation or disconnect preserves a durably journaled command attempt and prevents half-published views. System ordering changes require explicit tests because observation before validation can misrepresent authority.

## Persistence and durability

Save slots, settings, command journals, replay directories, maps/atlases, and package manifests are versioned, bounded, shape/hash checked, and atomically replaced where durable. Journals use restrictive modes, process locking, safe paths, write/flush/rename semantics, and exact immutable command identity. Corrupt slots are isolated rather than overwriting good state. A local journal proves attempted submission continuity only; a server receipt/snapshot and owning authority must confirm acceptance and state.

## Idempotency, retry and failure taxonomy

An exact journaled command can be retried with identical request/order bytes after recoverable transport loss. Altered duplicates, crossed account/campaign/match/generation/revision/build/protocol, snapshot/base/hash mismatch, and stale cursors fail closed and require resync or user-visible recovery. Asset/save/replay corruption, unsupported versions, queue full, timeout, TLS/HTTP/WebSocket error, settlement ambiguity, and local I/O have distinct actionable statuses. Offline fallback is explicit and cannot masquerade as canonical online or completed settlement.

## Versioning and migration

Client binary/build, asset/map schema, content/rules, save/settings/journal/replay, RTS/online/economy protocol, and package/update channel are separate identities. Persisted format changes require readers/migrations and rollback behavior. Wire changes require compatibility matrices and server/Nakama conformance. Input/UI semantic changes require user-facing migration notes and accessibility regression. Supported OS/render/audio/input backends and signing/update/rollback policies are recorded per release candidate.

## Resource and performance budgets

Frame/update/render/input/network timing, worker queue depth, HTTP/WebSocket body/frame, snapshots/deltas, assets/maps/atlases/audio, save/replay/journal size, entity counts, and shutdown time are bounded. Frame evidence distinguishes 30/60 FPS targets, input latency, network processing, and software-rendered diagnostic hosts. Loading or reconnect storms cannot block the UI indefinitely. Benchmarks and smoke captures bind exact hardware/driver/backend/resolution/build and report p50/p95/p99/stalls rather than a single average.

## Observability and security

Player-visible status states the current phase/objective/action plus offline/compatibility/canonical and local/pending/verified economy posture. Engineering telemetry binds build/protocol/rules/content, platform/backend, frame/input/network timing, queue/state transitions, reconnect/resync reason, save/replay/journal integrity, and bounded errors. Tokens/secrets/private paths and unnecessary personal data are excluded. Assets and packages are path/link/mode/hash/license verified before use; untrusted server/client data cannot create completion or wallet authority.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-first-contact/src/`, assets, packaging, and client scripts. Normative neighbors include `GAME_STATUS.md`, the closed-loop/release-gate documents, online/campaign/RTS module designs, runtime configuration, and human validation issue/runbook. Tests cover plugin/system order, input profiles, maps/assets, closed-loop campaign, local/attached authority separation, snapshots/deltas/reconnect, SIGKILL command-journal recovery, replay, corrupt files, frame stalls, packaging/install, and supported platforms. Automated screenshots cannot satisfy independent human evidence.

## Change checklist and open work

Before a client change: identify system-order/thread/blocking/state/persistence/protocol/accessibility effects; update view/state diagrams and migration notes; add keyboard/mouse/low-motion/high-contrast/subtitle/compact/wide tests; verify no authority/economy overclaim; measure frame/queue/resource impact; and update package/platform matrices. Open work includes decomposing large campaign flow/UI files into true modules/view models, publishing the Bevy schedule diagram, Windows/macOS signing/update paths, independent human sessions, and Nakama canonical reconnect comprehension.
