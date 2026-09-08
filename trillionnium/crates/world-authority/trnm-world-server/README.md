# trnm-world-server

Status: fixture/file-backed development and cutover smoke candidate  
Owner: Trillionnium World domain-service runtime

## Purpose

This crate wires the World-domain API to fixture identity/session/account/ledger/evidence/metrics adapters, a file-backed development repository, HTML/JSON smoke surfaces, and a minimal standalone TCP server used for deterministic parity, restart, cutover, and review exercises.

## Authority and non-goals

The current server is not production-capable and must not be publicly routed. Fixture sessions and ledger receipts are not security or custody evidence. The server does not replace Nakama canonical online admission/order/recovery/signing, CEX ledger authority, Chain finality, or Integration qualification. Production cutover requires durable PostgreSQL and real least-privilege adapters.

## Public contracts

The surface includes fixture adapter implementations, home/command/route/map/tactics/account/full-split builders, development repository and restart/reload behavior, minimal HTTP/TCP parsing and response helpers, and stable server/dev-runtime/repository/browser-parity contract identities. Every fixture output labels its source and non-production posture.

## State and invariants

One fenced World aggregate writer owns a revision at a time. Command success is persisted only after complete validation; rejection preserves prior state. File-backed data is bounded, safe-path, atomically replaced, and acceptable only for tests. Production state must use the catalog-verified PostgreSQL contract, stable IDs/hashes, epochs, immutable events, and no dual writer with legacy CEX mutation.

## Dependencies and boundaries

The server depends on World API/command/domain/map/projection/UI crates. Runtime adapters implement identity, repository, ledger, evidence, and metrics without leaking CEX/Nakama internals into domain packages. Network handlers cannot bypass application validation. Production routing, TLS, secrets, service identity, database pools, and observability are external deployment adapters.

## Failure and recovery

Malformed requests, unsupported routes/contracts, invalid commands, repository corruption, stale revision/epoch, adapter failure, and oversized/partial I/O fail closed with bounded responses. Restart reloads an exact valid state; ambiguous or corrupt files are quarantined. Production recovery uses PostgreSQL catalog/fencing/replay and exact reconciliation, never fixture acceptance.

## Testing and evidence

Tests cover adapter separation, command/state parity, full-split responses, safe HTML, bounded request parsing, file save/reload/restart, corrupt/truncated state, source provenance, no external path dependencies, and PostgreSQL cutover hostile matrices. Cross-host, TLS/public edge, identity, custody, endurance, backup/PITR, and human evidence remain open.

## Compatibility and change control

Server/API/repository/database/event versions are separate. Any route, mutation, persistence, epoch/fence, fixture, or adapter change requires contract tests, migration/rollback, threat/runbook updates, exact-head evidence, and review. Fixture code must not become an implicit production default.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-server-design.md`](../../../../docs/modules/world-authority/trnm-world-server-design.md).
