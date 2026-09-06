# trnm-online-protocol

Status: current candidate  
Owner: Trillionnium World compatibility wire vocabulary

## Purpose

This crate defines request, response, snapshot, delta, command-receipt, reconnect, product, operations, and production-status types used by the World-local compatibility enclave and native client.

## Authority and non-goals

The crate is compatibility vocabulary, not the target canonical online authority. Nakama owns target participant admission, canonical total order, idempotency, restart recovery, archive roots, and completion signing. Types in this crate must not be treated as proof of public deployment, Chain finality, or wallet settlement.

## Public contracts

The tree currently retains explicit V2 and V3 authority/build identities plus stream, product, operations, and production protocol versions. Protocol version is distinct from build identity. Requests bind player/account/campaign/match identities, expected revisions, per-player input sequence, observed tick, and typed RTS order as applicable.

## State and invariants

The crate is stateless. Wire material must have bounded identifiers and collections, stable enum spellings, explicit optional-field behavior, and crossed protocol/build rejection. Full snapshots, deltas, receipts, and reconnect cursors must bind exact sequence, revision, tick, generation, and hash relationships.

## Dependencies and boundaries

Only serialization, deterministic collections, and RTS order types are allowed. The crate must not open sockets, access databases or files, read credentials, or perform authority decisions. Clients and servers implement transport and authentication around these types.

## Failure and recovery

Unknown or crossed versions, malformed identities, sequence gaps, altered duplicates, hash mismatch, stale generation, truncation ambiguity, and unsupported capabilities fail closed with stable machine errors. Clients resync from an exact full snapshot when continuity cannot be proven.

## Testing and evidence

Required coverage includes serialization fixtures for every message class, full/delta/reconnect negative vectors, pagination and truncation, duplicate races, crossed build/protocol combinations, bounded frames/queues, old-reader behavior, and client/server conformance on the same exact component lock.

## Compatibility and change control

Breaking wire changes require a new protocol identity and explicit admission matrix. V2/legacy support must have an owner, usage inventory, retirement date, rollback policy, and tests. Canonical Nakama APIs must be defined in the owning repository rather than expanded here.
