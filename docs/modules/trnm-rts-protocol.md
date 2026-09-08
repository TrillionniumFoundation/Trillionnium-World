---
status: current-candidate
owner: Trillionnium World deterministic RTS command vocabulary
last_reviewed: 2026-09-08
review_due: 2026-10-08
release_effect: none-by-documentation
---

# trnm-rts-protocol technical design

This document describes the current review boundary and the next accepted target for the module. It is subordinate to `PROJECT_BOUNDARY.md`, accepted ADRs, `CURRENT_PLAN.md`, normative schemas and executable tests. Statements marked as gaps are not implemented or release evidence.

## Current implementation

A stateless Rust contract for frame orders, order kinds, stances, targets, unit/control-group selections, queue/job controls, validation and stream fingerprints. The opt-in strict decoder enforces a bounded canonical JSON envelope while the legacy serde reader remains for replay compatibility.

## Responsibilities and non-responsibilities

Defines command shape and strict intake only. Player identity, authorization, controller ownership, canonical sequence, durable idempotency and match authority belong to the caller/Nakama.

## Public interfaces and data contracts

`trnm_rts_order_protocol_v1` is the legacy vocabulary. `trnm_rts_order_intake_v1` is the strict wire policy exposed through `strict::decode`, returning a privately constructed `ValidatedOrder`. The field dictionary, command matrix, error precedence and vectors are normative neighbors.

## State model and invariants

No mutable state is owned. Strict intake rejects unknown/duplicate fields, positional arrays, malformed UTF-8/types, unsupported versions, empty/oversized IDs, duplicate subjects and missing required targets. Subject order is preserved. Successful strict decoding must serialize to the legacy command bytes without semantic drift.

## Dependency direction

Uses deterministic serialization, collections and hashing only. It must not depend on Bevy, campaign storage, filesystem, database, environment, wall clock, network or credentials. Transports must enforce body limits before buffering and cannot silently fall back to the permissive legacy parser.

## Concurrency and cancellation

Decoding is synchronous, side-effect free and reentrant. There are no locks, tasks or global mutable caches. Parallel decoders are allowed because results depend only on input bytes and intake version. Diagnostics remain deterministic regardless of thread scheduling.

## Durability and recovery

The crate persists nothing. Authorities durably bind accepted raw/canonical bytes, intake version, player/controller identity and canonical sequence. Replays retain their declared legacy reader version. A rejected envelope is never converted into a different command for recovery.

## Failure, retry and idempotency

Failures return stable codes with deterministic precedence and no reflected unbounded payload. Invalid bytes cannot fall back to another version. Authentication/sequence gaps, duplicates and reconnect are separate authority errors. Retrying identical invalid bytes under the same policy yields the same rejection.

## Versioning and migration

Intake policy, order vocabulary, simulation rules and replay encoding have independent versions. Limit, type or error-precedence changes require a new intake version and vectors. Enum spelling/fingerprint changes require protocol migration. Legacy reader removal needs saved replay inventory and retirement evidence.

## Resource budgets and performance

Raw envelopes are capped at 128 KiB, IDs at 160 UTF-8 bytes, selections at 256 subjects and labels at 256 UTF-8 bytes. Parsing depth, collection counts and diagnostics are bounded. Runtime adapters must reserve bounded queue capacity and reject before simulation mutation.

## Observability and security

Return code, intake version, payload length and digest may be recorded; raw player payload is not logged by default. Security review covers parser differentials, duplicate keys, Unicode/escape ambiguity, resource amplification and authority-field smuggling.

## Verification traceability

The frozen corpus covers every command shape, boundaries, diagnostics, legacy compatibility and serialization preservation. A deterministic raw-byte differential matrix is checked by an independent interpreter and the actual Rust decoder. Promotion additionally requires route-level adoption and rejected-command state isolation.

## Known gaps and change checklist

Complete adapter adoption in client, World compatibility server and Nakama; publish a caller inventory and legacy retirement date; add fuzzing and cross-language corpus execution; and prove body-limit enforcement before buffering. Changes require synchronized schema, vectors and runtime conformance evidence.
