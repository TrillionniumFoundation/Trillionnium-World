# trnm-first-contact

Status: pre-alpha / technical alpha candidate  
Owner: Trillionnium World native client and player experience

## Purpose

This crate is the native Bevy client for the RPG-to-RTS-to-debrief-to-town loop. It owns presentation, input, campaign shell and UI, authored map/atlas/audio loading, deterministic simulation adaptation, local settings and save slots, replay inspection, online compatibility transport, command journaling, and player-facing evidence adapters.

## Authority and non-goals

The client is untrusted for canonical online state, participant identity, completion, wallet balance, settlement success, and finality. Local saves and journals provide offline state and retry continuity only. The client may emit typed commands and display verified snapshots/receipts; it may not manufacture authority evidence.

## Runtime composition

`FirstContactLivePlugin` wires asset/audio loading, campaign flow, campaign UI, HUD, map loading, online compatibility client, command journal, renderer, simulation adapter, frame timing, and view math. The ordered Bevy system chain is correctness relevant: input produces typed intents, adapters submit them, deterministic state advances, settlement is staged, and UI/render/audio observe the resulting state.

## State and invariants

Offline campaign state is mutated only through `trnm-campaign-core`. Battle simulation is mutated only through validated `trnm-rts-protocol` orders. Online-attached mode consumes server snapshots and disables local authority simulation. Save, settings, command journal, replay, map, and atlas files are versioned, bounded, hash/shape checked, and atomically replaced where durable.

## Dependencies and boundaries

Bevy owns presentation scheduling; Rodio owns local audio; HTTP/WebSocket adapters are bounded transport only. Domain semantics stay in RPG, campaign, RTS protocol/simulation, economy, and online protocol crates. Network work must not block the render/update thread; cross-thread results return as typed bounded events.

## Failure and recovery

Missing or corrupt assets, crossed protocol/builds, invalid snapshots, journal conflicts, unsupported saves, or uncertain economy results fail closed with actionable player status. Exact duplicate commands may be retried; altered duplicates are rejected. Offline fallback must be explicit and may not masquerade as canonical online operation.

## Testing and evidence

Required coverage includes plugin/system ordering, asset and map validation, campaign closed loop, online snapshot/reconnect behavior, command-journal crash recovery, replay verification, input profiles, frame-stall budgets, package/install paths, and human accessibility sessions. Automated screenshots and source tests do not satisfy independent human evidence.

## Compatibility and change control

Changing system order, input semantics, save/replay interpretation, asset schema, online protocol, or presentation of authority/economy state requires regression tests and migration notes. Supported operating systems, render/audio backends, signing, and update/rollback policies must be recorded per release candidate.
