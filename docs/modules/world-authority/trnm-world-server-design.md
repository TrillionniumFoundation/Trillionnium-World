---
status: current-candidate
owner: trillionnium-world-server-runtime
module: trnm-world-server
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# trnm-world-server detailed design

## Scope and authority

This crate wires the World-domain API to fixture identity/session/account/ledger/evidence/metrics adapters, a bounded file-backed development repository, JSON/HTML smoke surfaces and a minimal standalone TCP/HTTP server. It exists for deterministic parity, source split, restart/reload, cutover and review exercises. It is not production-capable or publicly routable. It cannot replace Nakama canonical online authority, CEX custody, Chain finality or Integration qualification.

## Public interfaces

The surface includes fixture adapter implementations, application builders for home/command/route/map/tactics/account/full-split responses, development repository save/load/restart behavior, bounded request parsing and response helpers, and stable server/dev-runtime/repository/browser-parity contracts. It consumes typed World API values and emits explicitly labelled fixture/development responses. Production identity, PostgreSQL, routing, TLS and observability adapters are not provided by fixture names or status strings.

## State model and invariants

One admitted writer epoch owns each mutable World aggregate. A command is committed only after identity/session/admission, expected revision, deterministic command and aggregate validation succeed. Rejection preserves the prior state. Fixture file data is bounded, safe-path, atomically replaced and acceptable only for tests. Production state must use the catalog-verified PostgreSQL contract, immutable event/request identities, stable aggregate revisions/hashes and database-enforced no-dual-writer fencing with legacy CEX mutation.

## Dependency direction

The server depends on World API, command, domain, map, projection and UI packages. Runtime adapters implement identity/session/account/repository/ledger/evidence/metrics without leaking their implementation into inner packages. Request handlers translate bounded wire input to API calls and cannot bypass application invariants. Production networking, database pools, credentials, CEX/Nakama clients, TLS/WAF/routing and system service management are separate reviewed adapters/deployment packages.

## Execution, concurrency and cancellation

The current server is deliberately small and bounded; production design admits one mutation per aggregate revision and isolates independent aggregates. Request/body/read/write/in-flight and connection lifetimes have explicit ceilings. No remote ledger or network call is held under a mutable World database transaction. Cancellation before commit releases temporary state. Ambiguity after repository commit is resolved by immutable request/event lookup. Shutdown stops admission, drains bounded work, persists or safely abandons private candidates and emits a terminal identity-bound summary.

## Persistence and durability

The file-backed repository writes only development fixtures using safe paths, ownership/mode checks and atomic replacement; corruption/truncation is rejected and quarantined. It is not a production fallback. Production uses PostgreSQL migrations, complete catalog fingerprint, writer epochs, immutable event and idempotency records, constraints/triggers/indexes/privileges and exact replay/reconciliation. Stable IDs/counts/canonical hashes are compared with the legacy CEX source before cutover. Backup/PITR verifies lineage and fences old primaries/processes.

## Idempotency, retry and failure taxonomy

A mutation request is durably bound to immutable actor/account/session, request ID, expected revision, command bytes and contract versions. Exact duplicates converge; altered duplicates and stale epochs/revisions fail. Failures include malformed/oversized request, unsupported route/contract, authentication/session/audience, authorization, domain rejection, repository conflict/corruption/ambiguity, ledger ambiguity/permanent rejection, evidence/metrics degradation, capacity/rate, shutdown/drain and internal integrity. Fixture success is never promoted to production evidence.

## Versioning and migration

Server transport, API/domain/command/projection/map/UI, repository/event/database, fixture/runtime, identity/session, ledger/evidence and release selectors are separate versions. A route, mutation, adapter, repository format, epoch/fence, response or error change requires versioned contracts, old/new compatibility, migration/rollback/PITR effects, hostile tests and component-lock updates. Moving from fixture/file to production adapters is an explicit cutover, not a configuration alias.

## Resource and performance budgets

Request line/header/body, connection, in-flight request, worker/task/queue, JSON depth, identifiers/text, aggregate/file, projection/fragment and response bytes are bounded. Production adapters additionally bound database pool/query/lock, ledger timeout/retry and evidence/metrics overhead. Benchmarks report p50/p95/p99 and resource saturation for command/read/restart at representative state sizes. Public routing requires load/abuse/capacity evidence, not merely a local listener.

## Observability and security

Logs/evidence bind server/API/domain/component versions, process/instance/host/writer epoch, request/actor/account/session/aggregate/revision, repository/ledger/evidence result and bounded code. Secrets, cookies, tokens, private keys, authorization headers, unsafe paths and unbounded payloads are excluded. Production requires workload identity or mTLS, distinct audiences/roles, least-privilege database grants, TLS/WAF/rate limits, backup encryption, incident response and privacy/retention controls. The fixture server is disabled from public promotion.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-server/src/lib.rs`. Normative material includes ADR-0003, all sibling module designs, `docs/architecture/cex-world-authority-cutover-v1.md`, authority/provenance contracts, three PostgreSQL SQL files and the root operations manual. `scripts/check-trnm-world-authority-cutover.sh`, PostgreSQL installation/black-box scripts and dedicated workflows cover source closure, adapters, split, restart/reload, provenance, schema/catalog and hostile clones. Cross-host/public/custody/human evidence is separate.

## Change checklist and open work

A change records authority, routing, state, transaction, persistence, idempotency, version, security and resource effects; updates API/schema/SQL/runbook/rollback; adds success plus malformed/duplicate/stale/cancel/restart/corruption/old-writer tests; and obtains exact-head review. Open work includes replacing all fixture adapters with production implementations, eliminating the file repository from production builds, authenticated Nakama/internal routing, CEX exact ledger lookup, immutable evidence/metrics, service deployment, PITR/failover, cross-host endurance and no-dual-writer cutover proof.
