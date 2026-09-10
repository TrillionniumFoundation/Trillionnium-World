# trnm-world-domain

Status: current cutover candidate  
Owner: Trillionnium World domain state

## Purpose

This crate defines deterministic World-domain aggregate types and authored fixture data: World nodes/edges/positions, characters, tasks, NPCs, skills/items, transition semantics, route state, and stable contract identities used by the command, projection, map, API, and server layers.

## Authority and non-goals

It owns World-domain shape and invariants only. It does not authenticate players, assign canonical online sequence, persist a production aggregate, open a network connection, execute CEX ledger operations, sign `MatchCompletedV1`, establish Chain finality, or qualify a multi-repository release. Nakama remains canonical online authority and CEX remains custody authority.

## Public contracts

The public surface is `WorldState` and its typed supporting records, stable contract/version constants, fixture constructors, canonical lookups, transition classifications, validation helpers, and deterministic content/state views. IDs are explicit machine keys; display text is not parsed back into authority. The domain contract and transition-semantics contract must accompany persisted or transported state.

## State and invariants

Aggregate validation preserves unique and nonblank IDs, valid node/edge/position references, bounded collections and text, deterministic ordering, valid character/task/NPC/skill/item references, and exact contract identities. A domain value is immutable from the crate's perspective until the command layer constructs and validates a candidate successor. Invalid fixture or decoded data is rejected rather than repaired silently.

## Dependencies and boundaries

The crate may use deterministic collections, serialization, and hashing. It must not depend on command/application/server/UI code, SQL, filesystem state, environment variables, wall-clock time, randomness services, HTTP, credentials, CEX internals, or Nakama internals. Every higher layer depends inward on this domain package.

## Failure and recovery

Constructors and validators return or expose bounded deterministic rejection for invalid IDs, references, bounds, contracts, or state shape. Recovery reloads a previously valid exact revision or applies an explicit migration; the crate never invents substitute content or treats an invalid aggregate as partially authoritative.

## Testing and evidence

Required tests cover fixture validity, ID/reference uniqueness, transition topology, bounded values, canonical serialization/hash stability, hostile malformed states, version rejection, and repeated deterministic construction. Source/unit evidence cannot prove PostgreSQL durability, online admission, deployment, custody, or public readiness.

## Compatibility and change control

Changing a field, stable ID, transition meaning, canonical encoding, fixture semantics, or invariant requires a new domain/rules/content revision, compatibility vectors, migration and rollback notes, downstream command/projection/API tests, and exact component-lock review. Retired IDs require inventory and a dated reader policy.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-domain-design.md`](../../../../docs/modules/world-authority/trnm-world-domain-design.md).
