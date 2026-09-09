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

This crate wires the World-domain API to fixture identity/session/account/ledger/evidence/metrics adapters, a development JSON repository, JSON/HTML smoke surfaces and a minimal standalone TCP/HTTP server. It exists for deterministic parity, source split, restart/reload, cutover and review exercises. It is not production-capable or publicly routable. It cannot replace Nakama canonical online authority, CEX custody, Chain finality or Integration qualification.

## Public interfaces

The surface includes fixture adapter implementations, application builders for home/command/route/map/tactics/account/full-split responses, development repository save/load/restart behavior, request/response helpers, and stable server/dev-runtime/repository/browser-parity contracts. The executable listener is additionally governed by `trillionnium_world_fixture_runtime_policy_v1`: startup requires `--allow-fixture-runtime`, a literal loopback `SocketAddr`, and a nonzero explicit port. Production identity, PostgreSQL, routing, TLS and observability adapters are not provided by fixture names or status strings.

## State model and invariants

One admitted writer epoch owns each mutable World aggregate. A command is committed only after identity/session/admission, expected revision, deterministic command and aggregate validation succeed. Rejection preserves the prior state. Fixture file data is acceptable only for tests and must never be selected by a production profile. Production state must use the catalog-verified PostgreSQL contract, immutable event/request identities, stable aggregate revisions/hashes and database-enforced no-dual-writer fencing with legacy CEX mutation.

## Dependency direction

The server depends on World API, command, domain, map, projection and UI packages. Runtime adapters implement identity/session/account/repository/ledger/evidence/metrics without leaking their implementation into inner packages. Request handlers translate bounded wire input to API calls and cannot bypass application invariants. Production networking, database pools, credentials, CEX/Nakama clients, TLS/WAF/routing and system service management are separate reviewed adapters/deployment packages.

## Execution, concurrency and cancellation

The listener is disabled unless the operator supplies the explicit fixture opt-in flag. Before `TcpListener::bind`, the binary parses a literal IPv4/IPv6 socket address, rejects whitespace/hostnames/port zero, and requires `ip().is_loopback()`. The current server remains a small single-process development surface; it is not a concurrency or public-edge implementation. Production design admits one mutation per aggregate revision, isolates independent aggregates, bounds request/read/write/in-flight lifetimes, stops admission before drain and never holds a mutable World transaction across remote ledger or network I/O.

## Persistence and durability

The current file-backed repository serializes development fixture state to JSON and can demonstrate reload continuity, but it is not crash-durable and receives no atomicity, fsync, backup, recovery-point or production evidence. Production profiles explicitly prohibit it. Production uses PostgreSQL migrations, complete catalog fingerprint, writer epochs, immutable event and idempotency records, constraints/triggers/indexes/privileges and exact replay/reconciliation. Stable IDs/counts/canonical hashes are compared with the legacy CEX source before cutover; backup/PITR verifies lineage and fences old primaries/processes.

## Idempotency, retry and failure taxonomy

A production mutation request is durably bound to immutable actor/account/session, request ID, expected revision, command bytes and contract versions. Exact duplicates converge; altered duplicates and stale epochs/revisions fail. Fixture-listener policy failures include missing opt-in, malformed or noncanonical socket literal, wildcard/public/non-loopback address and port zero. Runtime failures additionally include malformed/oversized request, unsupported route/contract, authentication/session/audience, authorization, domain rejection, repository conflict/corruption/ambiguity, ledger ambiguity/permanent rejection, evidence/metrics degradation, capacity/rate, shutdown/drain and internal integrity. Fixture success is never promoted to production evidence.

## Versioning and migration

Server transport, fixture-listener policy, API/domain/command/projection/map/UI, repository/event/database, identity/session, ledger/evidence and release selectors are separate versions. A route, mutation, listener admission rule, adapter, repository format, epoch/fence, response or error change requires versioned contracts, old/new compatibility, migration/rollback/PITR effects, hostile tests and component-lock updates. Moving from fixture/file to production adapters is an explicit cutover, not a configuration alias.

## Resource and performance budgets

The fixture listener currently has a small fixed read buffer and is not credited as a complete HTTP implementation; public routing is prohibited regardless of local performance. Production adapters must explicitly bound request line/header/body, connection, in-flight request, worker/task/queue, JSON depth, identifiers/text, aggregate/projection/response bytes, database pool/query/lock, ledger timeout/retry and evidence/metrics overhead. Benchmarks report p50/p95/p99 and saturation at representative state sizes. A local loopback listener cannot satisfy public capacity evidence.

## Observability and security

Every listener start emits the fixture classification, policy contract, loopback bind and `production_authorization=not_granted`. Logs/evidence must exclude secrets, cookies, tokens, private keys, authorization headers, unsafe paths and unbounded payloads. Production requires workload identity or mTLS, distinct audiences/roles, least-privilege database grants, TLS/WAF/rate limits, backup encryption, incident response and privacy/retention controls. Any attempt to bind the fixture listener to `0.0.0.0`, `[::]`, a hostname or a non-loopback address fails before socket creation.

## Test and evidence traceability

Primary source is `trillionnium/crates/world-authority/trnm-world-server/src/lib.rs`; executable policy source is `trillionnium/crates/world-authority/trnm-world-server/src/runtime_policy.rs` and `src/main.rs`. Normative material includes ADR-0003, all sibling module designs, `docs/architecture/cex-world-authority-cutover-v1.md`, authority/provenance contracts, three PostgreSQL SQL files and the root operations manual. `scripts/check-trnm-world-authority-cutover.sh`, all-target Rust tests, PostgreSQL installation/black-box scripts and dedicated workflows cover source closure, listener policy, adapters, split, restart/reload, provenance, schema/catalog and hostile clones. Cross-host/public/custody/human evidence is separate.

## Change checklist and open work

A change records authority, routing, listener policy, state, transaction, persistence, idempotency, version, security and resource effects; updates API/schema/SQL/runbook/rollback; adds success plus malformed/duplicate/stale/cancel/restart/corruption/old-writer tests; and obtains exact-head review. Open work includes replacing all fixture adapters with production implementations, eliminating the JSON repository and hand-written TCP server from production builds, authenticated Nakama/internal routing, CEX exact ledger lookup, immutable evidence/metrics, service deployment, PITR/failover, cross-host endurance and no-dual-writer cutover proof.
