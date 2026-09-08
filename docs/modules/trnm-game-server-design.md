---
status: current-candidate
owner: trillionnium-world-server
module: trnm-game-server
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-game-server detailed design

## Scope and authority

`trnm-game-server` is the bounded `world_legacy_local_alpha` compatibility enclave. It provides laboratory/migration online authority, PostgreSQL persistence/migrations, host journals, private operations, product/compatibility APIs, settlement capture/apply integration, an independently deployed settlement worker, and signer protocol support. It is not the target public canonical online runtime and cannot own Nakama completion keys, Chain finality, CEX custody, or public-market enablement.

## Public interfaces

The exposed surface includes bounded player/compatibility HTTP and WebSocket routes, readiness/status, moderator/operations endpoints, product/lobby/match/command/snapshot/reconnect values, database migration/procedure contracts, settlement worker and entitlement signer protocols, and service bootstrap binaries. Normative wire/database behavior lives in the HTTP, WebSocket, error, compatibility, PostgreSQL, settlement, identity, and operations documents. Handlers translate authenticated/admitted requests into application commands; they do not directly manufacture domain or ledger success.

## State model and invariants

Correctness ownership is partitioned into authority foundation/actor runtime, configuration/migrations, terminal recovery/journals, fleet fencing, identity/authorization, application services, HTTP routing, readiness, product APIs, campaign persistence, settlement, and operations. Public cursors advance only after durable command/terminal publication. Database/host/instance/lease epochs fence stale writers. Match/member/campaign/settlement identities are immutable. Remote success and campaign apply are separate durable states. Missing/ambiguous evidence makes readiness false.

## Dependency direction

Domain/protocol crates sit below application services; HTTP/PostgreSQL/CEX/signer/filesystem adapters depend inward; runtime bootstrap wires them without owning business rules. The server may use Axum, Tokio, SQLx/PostgreSQL, bounded HTTP/WebSocket transports, journals, and game-domain crates. Domain code must not import adapters. Request handlers must not bypass application invariants. Sibling repository path dependencies, private Nakama keys, and direct CEX implementation imports are forbidden.

## Execution, concurrency and cancellation

Each match has a bounded actor/initialization lifecycle and one admitted command-persistence lane. Database mutations follow the documented global lock order. No signer/CEX/network operation may execute while mutable match/member/campaign/settlement rows are locked. Settlement uses short capture/claim/apply transactions with transaction-free remote execution. Task/queue/socket/request/body/time budgets are explicit; cancellation guards release reservations and preserve durable/public cursors. Shutdown stops admission before draining actors/workers and bounded joins.

## Persistence and durability

PostgreSQL is the durable compatibility store; append-only migration checksums, constraints, triggers, functions, owners, privileges, indexes, and catalog fingerprints are part of the contract. Hot/cold host journals witness published ticks, terminal ACKs, abandonment, generations, and database lineage; corruption or uncertain fsync poisons readiness. Terminal publication atomically binds result/settlement/projections/ACK. Campaign and settlement apply use exact revision/state-hash CAS. PITR/failover requires timeline/system-identifier and old-primary isolation.

## Idempotency, retry and failure taxonomy

Command, terminal, settlement capture/job/remote request/receipt/apply, replay, and operator decision identities are immutable and versioned. Exact duplicates converge after full binding checks; altered duplicates fail. Response loss, malformed success, conflict, or timeout remains ambiguous and invokes exact lookup-before-submit. Every worker mutation requires a live exact lease/generation. Failures distinguish malformed/unsupported, authentication/authorization, stale revision/generation/lease, capacity/backpressure, integrity/hash, migration/catalog, journal/durability, remote ambiguity/permanent rejection, quarantine, and operator-policy denial.

## Versioning and migration

Endpoint/message protocol, database migration, journal/manifest, authority generation, product/operations/production protocol, World rules/content, signer entitlement, and release build are independent identities. Migrations are append-only and checksum/catalog bound; repairs use new versions. Breaking wire/database/lock-order/journal/settlement behavior requires new contracts, old/new readers, rollout/rollback/PITR effects, hostile tests, and component-lock updates. Retirement requires Nakama cutover, active-match drain, receipt reconciliation, rollback rehearsal, and route disablement.

## Resource and performance budgets

The server enforces request-body, frame, identifier, queue, actor, connection, command, receipt-gap, snapshot, journal, database pool/query/lock, settlement concurrency/attempt/backoff, diagnostic, and shutdown limits. The CEX/signer success-body policy is bounded at 2 MiB with redirects denied. Capacity/readiness reserves headroom for probes and terminal recovery. Benchmarks and load evidence report endpoint/command/persistence/snapshot p50/p95/p99, pool/lock saturation, queue age, match initialization, settlement backlog, and resource exhaustion without hiding failures through unlimited retries.

## Observability and security

Metrics/logs bind instance/host/epoch/region/build/protocol/rules/content/database lineage, match/campaign/job/cursor identities, bounded result codes, queue/pool/lock/latency, journal and settlement states. Tokens, private keys, authorization headers, unbounded bodies, and sensitive player data are redacted. Distinct game/moderator/worker/signer audiences and least-privilege database roles are mandatory. Public routing, KMS/HSM, mTLS/workload identity, WAF/DDoS, privacy/retention, moderation/support, and custody remain external evidence gates.

## Test and evidence traceability

Primary source is `trillionnium/crates/trnm-game-server/src/`, migrations, tests, and binaries. Normative neighbors include the PostgreSQL contract/catalog/lock order, settlement lifecycle/outbox/runbooks, service identity/secrets, threat model, HTTP/WebSocket/error/compatibility contracts, and testing strategy. Required evidence covers all-target Rust tests/Clippy, PostgreSQL black-box schema/procedures/privileges, two-worker/stale lease, response loss/kill/cancel, journal corruption, shutdown/drain, PITR/timeline/old primary, package/supply chain, exact-head and prospective merge.

## Change checklist and open work

For a change: identify authority/state/lock/durability/idempotency/version/resource effects; update the owning module contract, SQL/wire schemas and runbooks; add success plus hostile/cancellation/restart tests; preserve read-only CI and exact evidence identity; and obtain affected-owner review. The hidden semantic build generator is retired. Remaining structural work is to replace ownership-labelled `include!` parts with true Rust `mod`/trait/visibility boundaries, narrow `AppState`, move SQL behind repositories, finish Nakama cutover/retirement, and obtain native/cross-host/production evidence.
