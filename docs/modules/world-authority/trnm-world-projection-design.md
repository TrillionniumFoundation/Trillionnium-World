---
status: current-candidate
owner: trillionnium-world-projection
module: trnm-world-projection
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-projection detailed design

## Scope and authority

This crate produces deterministic, rebuildable World read models from a validated aggregate and source-versioned route/event records. It owns home views, route previews/artifacts, task-graph/story and commerce views, UI command-target descriptors and stable projection contract identities. A projection is never a mutation, admission, ordering, completion, ledger or finality decision. It can describe only state present in authoritative input.

## Public interfaces

The public surface includes `WorldHomeProjection`, `WorldRouteRecords`, preview item, task graph, story, artifact and command-target types; projection/fragment contract constants; and pure functions such as home projection, route artifact construction, command-target mapping and full-split JSON views. Outputs carry source-of-truth, component contract/revision and stable machine IDs so API/UI caches can reject stale or crossed material.

## State model and invariants

The crate has no mutable state. Identical canonical aggregate and route records yield byte-equivalent ordered projections. Output ordering and tie-breaks are explicit and deterministic. Every referenced ID originates from validated input; missing or duplicate source records, impossible status combinations, unsupported contracts and exceeded bounds fail or produce an explicitly non-authoritative partial view according to the contract. Projection code cannot create a task, item, balance, receipt or success absent from input.

## Dependency direction

The projection crate depends on domain records and deterministic serialization/collections. It does not depend on mutation commands for hidden side effects, HTML rendering, HTTP/server, SQL/files, environment/time, credentials, CEX or Nakama implementations. API and UI layers consume typed projections. A repository may cache serialized projections as source-versioned derived data, but the domain/application layers never read those caches to determine correctness.

## Execution, concurrency and cancellation

Projection is synchronous, pure and safe for concurrent immutable callers. It starts no tasks and acquires no locks. A caller snapshots one committed aggregate revision plus a consistent route/event cursor, derives a private projection and publishes it only if the source identity remains current. Cancellation drops the private result. Long-running or paged projection is bounded and cannot hold a mutable aggregate/database transaction or block unrelated accounts.

## Persistence and durability

All projection data is disposable and rebuildable. A durable cache stores exact source aggregate revision/hash, route/event cursor, projection version and serialized digest. Cache loss does not lose authority. A stale, corrupt or crossed cache is discarded, not used to repair state. Production database schemas may materialize views only if refresh/invalidation and source-version checks are explicit. Fixture JSON in tests is not production evidence.

## Idempotency, retry and failure taxonomy

Rebuilding the same source tuple is naturally idempotent. Cache writes use a deterministic projection key and may replace only the same or newer source revision under policy. Failures distinguish unsupported projection/source contract, invalid source reference/status, inconsistent cursor, exceeded resource limit, serialization/integrity error and stale-source publication. A retry re-reads an exact committed source rather than combining results from different revisions.

## Versioning and migration

Projection schema, ordering, status vocabulary, source domain/event version, UI fragment version and API transport version are distinct. Changing a field, ordering, command-target meaning, omission/default rule or derived semantic requires a new projection version or explicit compatible extension, old/new consumer fixtures, cache invalidation/rebuild behavior and API/UI updates. No projection version may add authority fields that are not signed or stored by the accountable owner.

## Resource and performance budgets

Route/event inputs, preview items, task graph nodes/edges, story records, text/metadata size, nested JSON depth and output bytes are bounded. Derivation must be linear or documented in bounded source size and use deterministic indexed collections to avoid accidental quadratic scans. Large account histories require pagination/materialized summaries. Benchmarks cover maximum valid inputs, rebuild latency, allocation/output bytes, cache hit/miss and concurrent independent accounts.

## Observability and security

Telemetry records projection contract, source aggregate revision/hash, source event cursor, item counts, output bytes/digest, cache result, duration and stable bounded failure code. It does not log credentials or unnecessary private content. User-authored text remains typed data and is escaped by the UI layer, not trusted as markup. A projection marked fixture/partial/stale cannot be displayed or transported as canonical completion, settlement or public-online evidence.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-projection/src/lib.rs`. Normative documents are `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md`, `docs/architecture/cex-world-authority-cutover-v1.md`, and the sibling domain/UI/API/server designs under `docs/modules/world-authority/`. Executable trace checks are `scripts/check-trnm-world-authority-documentation.py` and `scripts/check-trnm-world-authority-cutover.sh`. Crate and server parity tests cover deterministic ordering, stable IDs, source binding, route/status combinations, missing/duplicate records, bounds, hostile strings and golden JSON. Browser, human and public-network evidence remains separate.

## Change checklist and open work

A change lists affected source/projection/API/UI versions; updates field/order/default/cache rules and examples; adds exact, stale, partial, hostile and limit tests; confirms no mutation/authority dependence; and measures rebuild/output cost. Open work includes generated JSON Schema, a field and derivation dictionary, durable projection-cache contract, pagination/backpressure rules, broader property/fuzz tests, and cross-client golden conformance on an immutable component lock.
