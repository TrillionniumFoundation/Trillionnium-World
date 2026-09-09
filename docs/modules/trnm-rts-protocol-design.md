---
status: current-candidate
owner: trillionnium-world-rts-protocol
module: trnm-rts-protocol
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-rts-protocol detailed design

## Scope and authority

`trnm-rts-protocol` defines the deterministic RTS command vocabulary, selections/control groups, targets, stances, queue/job controls, validation, fingerprints, and the opt-in strict JSON intake envelope. It checks wire and command shape only. It does not authenticate a player, authorize controller ownership, assign canonical online order, persist idempotency, simulate a battle, render, sign results, or settle value. Nakama/calling authority must perform admission independently.

## Public interfaces

`RtsFrameOrder` and related enums remain the legacy serialized command vocabulary under `trnm_rts_order_protocol_v1`. `strict::decode(&[u8])` implements `trnm_rts_order_intake_v1` and returns a privately constructed `ValidatedOrder` or stable `IntakeError`; `as_order()` borrows and `into_order()` transfers the validated command. The protocol document defines the full field dictionary, 28-command shape matrix, error precedence, schema/vectors, and adoption checklist. Fingerprints bind intended serialized command material, not player authority.

## State model and invariants

The crate is stateless. Strict intake rejects duplicate or unknown fields at every supported object boundary, positional array substitutions, malformed UTF-8/types, unsupported versions, empty/oversized identifiers, duplicate subjects, missing/forbidden targets, and derived enum object encodings. It preserves subject order and legacy serialized bytes after success. Raw envelopes are capped at 128 KiB, identifiers at 160 UTF-8 bytes, selections at 256 subjects, and labels at 256 UTF-8 bytes.

## Dependency direction

The protocol may depend on deterministic serialization, bounded collections, and hashing only. It must not import Bevy, campaign storage, simulation state, filesystem/environment/wall-clock/randomness, SQL, HTTP, credentials, or Nakama/CEX implementation code. Simulation, client, World compatibility server, and Nakama adapters may consume the protocol. They must not bypass strict intake for untrusted new traffic once adoption is declared.

## Execution, concurrency and cancellation

Decoding is synchronous, deterministic, side-effect free, and bounded by the envelope and collection ceilings. It starts no tasks, owns no locks, and publishes no command until full validation succeeds. Cancellation simply drops temporary values. Concurrent decoders are independent. Transport adapters enforce body/frame limits before buffering and must isolate a rejected command from match, queue, replay, cursor, resource, and telemetry mutation except for an explicitly non-authoritative rejection metric.

## Persistence and durability

This crate persists nothing. Authorities may durably store raw bytes, the accepted protocol/intake version, canonical decoded command, fingerprint, actor/session/sequence/idempotency identity, and admission result. Replays/saves using the permissive legacy Serde reader remain versioned compatibility material; direct legacy deserialization is not equivalent to strict intake. A persisted command is executable only after the owning authority verifies all external identity, ownership, phase, cost, and sequence fences.

## Idempotency, retry and failure taxonomy

Intake errors are stable and non-mutating. Repeating identical invalid bytes under the same version yields the same rejection. The decoder does not decide whether a valid duplicate is idempotent; Nakama or the compatibility server binds immutable command identity/sequence and distinguishes exact replay from altered conflict. Error precedence must be frozen so independent implementations agree on one code for multi-defect inputs. There is no fallback from rejected strict intake to permissive legacy parsing.

## Versioning and migration

Order vocabulary, strict-intake policy, simulation ruleset, and release/build provenance are separate versions. Changing field/type rules, resource ceilings, unknown-field behavior, error precedence, or canonical intake shape requires a new intake version and updated positive/negative/differential vectors. Changing enum spellings, fingerprint material, or gameplay meaning follows the order/ruleset process. Legacy support needs usage inventory, owner, compatibility matrix, rollback policy, and dated retirement gate.

## Resource and performance budgets

The decoder must reject oversized raw input before deep parsing, bound object depth/fields/strings/selections, and avoid quadratic duplicate/unknown-field checks. Its worst-case time and allocation are measured over the complete boundary corpus, Unicode identifiers, maximum selections, and hostile multi-error input. HTTP/WebSocket adapters use equal or stricter limits. A larger budget requires denial-of-service analysis, benchmark evidence, updated schema/vectors, and compatible server/client/Nakama admission.

## Observability and security

Diagnostics expose stable static error codes and bounded field paths, never reflect complete untrusted payloads or secrets. Metrics separate transport-size rejection, syntax/type/version/unknown/duplicate/identifier/selection/target failures, accepted commands, and downstream authority rejection. Strict decoded keys and enum spellings prevent alternate encodings from smuggling authority meaning. The decoder must be fuzzed for panic, allocation, depth, Unicode, duplicate-key, and crossed-encoding behavior.

## Test and evidence traceability

Primary sources are `trillionnium/crates/trnm-rts-protocol/src/lib.rs`, `src/strict.rs`, `tests/strict_intake.rs`, and `examples/strict_intake_oracle.rs`. Normative material is `docs/protocol/trnm-rts-order-intake-v1.md`, its JSON Schema/vectors, and `docs/protocol/trnm-rts-intake-differential-v1.md`. The frozen corpus, independent Python checker, deterministic 361-case differential matrix, actual Rust oracle, fuzz/property tests, and runtime route isolation must agree on the same exact component lock.

## Change checklist and open work

For any change: classify order vs intake vs simulation semantics; update schema, field dictionary, matrix, precedence and vectors; prove serialization/fingerprint compatibility; add raw-byte boundaries and fuzz cases; update every runtime adapter and cross-repository lock; document retirement/rollback; and obtain exact-head execution. Open work is complete runtime adoption of strict intake, explicit removal date for direct permissive parsing, Nakama/client/server conformance, and non-empty hosted prospective-merge evidence.
