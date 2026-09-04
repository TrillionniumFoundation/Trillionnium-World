# trnm-rts-protocol

Status: current candidate; strict-intake Rust execution and adapter adoption pending  
Owner: Trillionnium World deterministic RTS command vocabulary

## Purpose

This crate defines the frame-order contract consumed by deterministic RTS simulation: order kinds, stances, targets, unit/control-group selections, queue/job controls, validation and stream fingerprints. It now also exposes an explicitly versioned, bounded JSON intake policy without changing old replay serialization.

## Authority and non-goals

The crate checks command shape, not player identity, permission, controller ownership, canonical online sequence or durable idempotency. It does not simulate, render, persist matches, sign results or settle wallets. Nakama owns canonical online admission/order. The caller must authenticate and admit input independently, even when intake returns a validated command.

## Public contracts

`trnm_rts_order_protocol_v1` remains the legacy command vocabulary. `strict::decode(&[u8])` implements the opt-in `trnm_rts_order_intake_v1` envelope and returns a privately constructed `ValidatedOrder` or stable `IntakeError`. `as_order()` borrows the result and `into_order()` transfers it. The full field dictionary, 28-command shape matrix, error precedence, resource ceilings, schema, vectors and adoption checklist are in `docs/protocol/trnm-rts-order-intake-v1.md` at the repository root.

## State and invariants

The crate is stateless. Strict intake rejects unknown/duplicate fields at envelope, order and tile boundaries, positional arrays, malformed UTF-8/types, altered versions, empty/oversized identifiers, duplicate subjects and missing required targets. It preserves subject order and the legacy command's serialized bytes after successful decoding. Raw envelopes are bounded at 128 KiB, identifiers at 160 UTF-8 bytes, selections at 256 subjects and labels at 256 UTF-8 bytes.

The legacy `RtsFrameOrder` Serde reader intentionally retains its prior permissive unknown-field behavior for compatibility and does not enforce all strict-intake limits. Calling it directly is not equivalent to strict intake. Existing runtime callers are not silently migrated by this library change. Map bounds, costs, cooldowns, phase validity and authority admission remain outside intake.

## Dependencies and boundaries

Only existing deterministic serialization, collections and hashing dependencies are used. The library must not depend on Bevy, campaign storage, environment variables, wall-clock time, randomness, database, filesystem, HTTP or credentials. Intake owns only temporary values; there are no mutable global variables, asynchronous tasks, locks, remote calls or durable writes. Transport adapters must reject oversized bodies before their own buffering.

## Failure and recovery

Intake failures publish no command or simulation mutation and return static codes, never reflected payloads. A rejection under one intake version cannot silently fall back to the legacy parser. Retrying identical invalid bytes under the same policy is not useful. Authentication, sequence gaps, duplicate identities and reconnect are handled by the owning authority, not inferred here.

## Testing and evidence

`tests/strict_intake.rs` consumes the shared raw JSON corpus and adds byte/count/Unicode boundaries, diagnostics, legacy-reader compatibility and serialization-preservation tests. `scripts/check-trnm-rts-intake-conformance.py` independently interprets that corpus and exercises the reference checker. Its success is not Rust execution or agreement. The read-only RTS-intake workflow defines tests for the exact head and, on PR events, the independently checked merge object. Hosted results, runtime adapter adoption and route-level rejected-command isolation remain open; local reference tests grant no public-online or release evidence.

## Compatibility and change control

Intake policy, order vocabulary, simulation ruleset and release provenance have separate versions. Changing intake limits, type rules or error precedence requires a new intake version and conformance vectors. Changing enum spellings, serialized/fingerprint material or gameplay meaning follows the separate order/ruleset version process. Old save/replay readers cannot be removed without migration inventory and retirement evidence. No production or canonical authority surface is enabled by this candidate.
