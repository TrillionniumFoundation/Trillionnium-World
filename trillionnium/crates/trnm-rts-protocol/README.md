# trnm-rts-protocol

Status: current candidate  
Owner: Trillionnium World deterministic RTS command vocabulary

## Purpose

This crate defines the lightweight frame-order contract consumed by the deterministic RTS simulation. It contains order kinds, stances, targets, unit/control-group selections, queue operations, job controls, and validation/fingerprinting needed to treat player input as deterministic data.

## Authority and non-goals

The crate validates command shape and identity; it does not execute simulation, choose AI actions, render controls, assign online global sequence, persist matches, or authenticate players. Canonical online admission and ordering belong to Nakama; the World-local use is compatibility-only.

## Public contracts

`trnm_rts_order_protocol_v1` is the current protocol identity. Serialized enum spellings and field meanings are stable wire material. Display labels must not be used as protocol identifiers.

## State and invariants

The crate is stateless. Orders must have bounded identifiers and collections, legal target combinations, valid quantities and coordinates, and deterministic fingerprints. A command ID or input sequence cannot be reused with altered order bytes. Unknown order kinds and unsupported fields fail closed.

## Dependencies and boundaries

Only deterministic serialization and hashing dependencies are permitted. The crate must not depend on Bevy, campaign persistence, wall clock, random services, database, filesystem, HTTP, or online credentials. `trnm-rts-sim`, clients, and compatibility adapters may depend on it.

## Failure and recovery

Validation errors return before simulation mutation. Retry reuses the same command identity and bytes. Sequencing gaps, duplicates, and reconnect behavior are owned by the online authority protocol and must not be guessed inside this crate.

## Testing and evidence

Required tests cover every order kind, invalid target/selection combinations, bounds, stable serialization and fingerprint vectors, duplicate identity conflicts, queue operations, stance transitions, and forward/backward compatibility fixtures. Fuzz/property tests must preserve parser and validation bounds.

## Compatibility and change control

Changing enum spelling, field meaning, numeric width, defaulting, fingerprint material, or validation semantics requires a new protocol version. Additions require old-reader behavior to be declared and tested; removals require a migration and retirement window.
