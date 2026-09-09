# trnm-world-map-provider

Status: fixture-only current cutover candidate  
Owner: Trillionnium World map/content provider

## Purpose

This crate provides versioned fixture World maps, map-provider status, map-pack manifests, OpenStreetMap attribution/licensing evidence shapes, sensitive-POI screening, modelling/accessibility/privacy gates, and deterministic map/runtime budget projections.

## Authority and non-goals

Only the in-repository fixture provider is enabled. Live Overpass, Geofabrik, vendor-tile, or public-network ingestion is fail-closed until rate limits, cache/derivative lineage, ODbL attribution, privacy/geofence/takedown policy, signatures, and operational evidence exist. The crate does not admit users, order commands, operate a tile service, or grant public map readiness.

## Public contracts

The surface includes provider-mode/status types, fixture world constructors, unsigned map-pack manifest and attribution/sensitive-POI/model/performance/accessibility/privacy JSON contracts, canonical map hashing, and stable provider/version constants. Provider status explicitly records fixture/live-ingestion/public-network posture.

## State and invariants

Map nodes/edges/positions and derived models preserve unique stable IDs, valid topology, bounded dimensions/counts/text, deterministic canonical payload/hash, attribution requirements, and source lineage. Fixture output is reproducible. A disabled provider mode remains disabled rather than silently falling back to network ingestion or claiming live freshness.

## Dependencies and boundaries

The crate depends on World-domain types and deterministic serialization/hashing. It does not perform network I/O, write databases/caches, inspect user location, read credentials, or import UI/server/CEX/Nakama implementations. Acquisition/import/signing/cache services are separate reviewed adapters.

## Failure and recovery

Unknown or unqualified modes fail closed with a stable reason. Missing attribution, invalid topology, stale/unsigned pack, sensitive-POI findings, exceeded density/performance budgets, or lineage mismatch blocks promotion. Recovery selects a previously signed valid pack or completes the external review; it never marks a fixture as live.

## Testing and evidence

Tests cover mode fail-closure, fixture determinism, canonical hash, topology, attribution/ODbL fields, sensitive terms, privacy/accessibility/model contracts, density/performance ceilings, and malformed packs. Real provider ingestion, public routing, jurisdiction, human map comprehension, and commercial licenses require independent evidence.

## Compatibility and change control

Provider mode, geodata contract, map-pack schema, content revision, modelling, privacy, attribution, and transport versions are separate. Changes require signed manifests/vectors, cache invalidation, migration/rollback, license/provenance review, and client/server compatibility.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-map-provider-design.md`](../../../../docs/modules/world-authority/trnm-world-map-provider-design.md).
