---
status: current-candidate
owner: trillionnium-world-map-provider
module: trnm-world-map-provider
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-map-provider detailed design

## Scope and authority

This crate owns the versioned fixture map/content provider and the machine contracts used to judge a future map pack: canonical map payload/hash, attribution and derivative lineage, sensitive-POI/privacy screening, accessibility/modelling quality, density/performance budgets and provider status. Current authority is fixture-only. It does not perform live public-network geodata ingestion, operate a tile/cache service, admit players, order commands or grant public map readiness.

## Public interfaces

The surface includes provider mode/status records, `cex_default_world_from_map_provider` and fixture state helpers, unsigned map-pack manifest and attribution/sensitive-POI/model/performance/accessibility/privacy JSON builders, canonical map hash/contract constants, and fail-closed readiness fields. A consumer can distinguish fixture, disabled-live, unsigned, attributed, reviewed and production-qualified states rather than interpreting file presence as readiness.

## State model and invariants

Fixture output is deterministic and valid under the World-domain contract. Stable node/edge/position/content IDs are unique and topology/reference checks pass. Manifest bytes bind provider/schema/content revision, source lineage, attribution, licensing and payload digest. Live mode remains disabled unless all required evidence is present. Sensitive locations, personal data, prohibited tags, invalid geometry/topology, unbounded density/size, missing attribution or unsigned/stale packs block acceptance.

## Dependency direction

The package depends inward on World-domain types plus deterministic serialization/hashing. It does not open network connections, read credentials or user location, write caches/databases, invoke UI/server/CEX/Nakama, or select a production provider. Acquisition, rate-limit, download, verification, signing, cache and takedown operations belong to separate adapters/services and feed an immutable reviewed pack into this crate's validation boundary.

## Execution, concurrency and cancellation

Fixture construction and pack validation are synchronous, deterministic and side-effect free. They own no tasks or locks. A future acquisition pipeline downloads into a private temporary location under bounded timeout/size/rate policies, verifies complete provenance/signature/schema/hash, and atomically promotes only a fully accepted pack. Cancellation or partial download publishes nothing. Concurrent refreshes are serialized by one pack-generation identity and cannot overwrite a newer approved pack.

## Persistence and durability

This crate performs no durable writes. Production packs are immutable content-addressed artifacts with signed manifests, source/license/provenance records, review decision and rollback selector. Caches are derivative and rebuildable. An installed pack is activated atomically and retains the prior approved pack for rollback. Missing source lineage, signature, attribution or exact payload digest invalidates the candidate. Fixture manifests remain explicitly unsigned and cannot be relabelled.

## Idempotency, retry and failure taxonomy

An acquisition/import request binds provider, source snapshot/version, parameters and expected digest. Exact retries may reuse downloaded bytes after verification; altered content under the same identity fails. Failure families include disabled/unqualified provider, timeout/rate/transport, oversized payload, invalid format/geometry/topology, duplicate/missing ID, attribution/license/provenance failure, sensitive-POI/privacy finding, signature/digest mismatch, stale revision and density/performance/accessibility/model gate failure. No failure falls back to live network silently.

## Versioning and migration

Provider, acquisition protocol, map-pack schema, canonical hashing, World content revision, modelling/accessibility/privacy policy, attribution/license policy, signature key and release selector are independent versions. Any semantic or canonical-byte change requires updated vectors/manifests, client/server compatibility, cache invalidation, migration/rollback, provenance/legal review and a dated retirement window. Stable World IDs cannot be reused for different locations/content.

## Resource and performance budgets

Download bytes/time/rate, archive entries/compression ratio, JSON depth, identifiers/text, nodes/edges/POIs, geometry points, world density, rendered entities, runtime memory/load time and query cost are bounded. Validation occurs before large allocation and rejects zip/path/symlink hazards. Benchmarks bind exact pack, client/server build and hardware profile. A larger map promise requires measured frame, simulation, server and CDN/cache capacity rather than a document-only claim.

## Observability and security

Evidence records provider/source snapshot, request/response status without secrets, cache/ETag, bytes, manifest/payload/signature digests, attribution/license, screening/model/accessibility results, activation/rollback selector and bounded failure code. Tokens and private endpoints are redacted. Privacy minimization, geofencing, removal/takedown, abuse response, KMS signing and public-edge controls are external accountable evidence. Fixture status is visibly separated from production.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-map-provider/src/lib.rs`. Normative material includes `docs/architecture/cex-world-authority-cutover-v1.md`, authority contract/provenance JSON, ADR-0003 and map/public evidence gates already present in `scripts/`. Crate tests and `scripts/check-trnm-world-authority-cutover.sh` cover fixture determinism/topology/hash, attribution, sensitive terms, privacy/accessibility/model/performance contracts and disabled live mode. Real provider/public-network/legal evidence remains open.

## Change checklist and open work

A change records provider/source/schema/hash/content/policy effects; updates manifests, vectors, provenance, license/attribution, screening and resource gates; adds hostile archive/geometry/privacy/signature tests; proves atomic activation/rollback; and obtains map, privacy, security and legal review as applicable. Open work includes a real bounded acquisition adapter, signed immutable pack pipeline, cache/derivative lineage, takedown procedure, public CDN capacity, human map comprehension and production license evidence.
