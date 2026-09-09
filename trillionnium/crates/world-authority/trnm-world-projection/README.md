# trnm-world-projection

Status: current cutover candidate  
Owner: Trillionnium World read-model projections

## Purpose

This crate derives deterministic, rebuildable home, route, task-graph, story, commerce, and UI-target projections from validated World-domain state and source-versioned route records.

## Authority and non-goals

A projection is a read model, never a mutation or authority decision. This crate does not accept player identity, order commands, persist canonical state, execute settlement, render unescaped HTML, sign completion, or establish wallet/finality truth. Nakama, World command/application, CEX, and Chain remain accountable for those domains.

## Public contracts

The public surface includes typed `WorldHomeProjection`, route preview/record/artifact structures, task graph/story views, command-target descriptors, stable projection/fragment contract constants, and pure projection functions. Every projection carries enough source/revision identity for consumers to reject stale or crossed data.

## State and invariants

Projections contain no hidden mutable state. Identical validated aggregate and route-record inputs produce byte-equivalent ordered output. Stable IDs are preserved; missing references, duplicates, invalid status combinations, unbounded text/counts, and crossed versions fail or are omitted only according to an explicit contract. A projection cannot create a task, balance, receipt, or completion that is absent from authoritative input.

## Dependencies and boundaries

The crate depends inward on domain values and deterministic serialization/collections. It must not import command mutation, UI rendering, server/network/database adapters, filesystem/environment/time, credentials, CEX, or Nakama implementations. UI/API layers consume the typed projection rather than parsing raw domain internals or display strings.

## Failure and recovery

Invalid or incomplete projection input returns a stable bounded failure or an explicitly partial/non-authoritative view. Recovery reloads/rebuilds from an exact authoritative revision. Cached projections are discarded on source-version mismatch; they are never used to repair domain state.

## Testing and evidence

Tests cover ordering, stable IDs, source/revision binding, missing/duplicate records, bounds, XSS-sensitive strings passed as data, route/status combinations, deterministic rebuild, and golden JSON. UI screenshots or public routing are not implied by projection tests.

## Compatibility and change control

Changing fields, ordering, status semantics, command-target meaning, source identity, or encoding requires a projection version, old/new consumer tests, cache invalidation/migration behavior, and UI/API updates. Public authority fields may not be added without an ADR.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-projection-design.md`](../../../../docs/modules/world-authority/trnm-world-projection-design.md).
