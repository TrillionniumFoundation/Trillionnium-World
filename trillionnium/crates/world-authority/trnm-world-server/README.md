# trnm-world-server

Status: fixture/file-backed development and cutover smoke candidate  
Owner: Trillionnium World domain-service runtime

## Purpose

This crate wires the World-domain API to fixture identity/session/account/ledger/evidence/metrics adapters, a file-backed development repository, HTML/JSON smoke surfaces, and a minimal standalone TCP server used for deterministic parity, restart, cutover, and review exercises.

## Authority and non-goals

The current server is not production-capable and must not be publicly routed. Fixture sessions and ledger receipts are not security or custody evidence. The server does not replace Nakama canonical online admission/order/recovery/signing, CEX ledger authority, Chain finality, or Integration qualification. Production cutover requires durable PostgreSQL and real least-privilege adapters.

## Public contracts

The surface includes fixture adapter implementations, home/command/route/map/tactics/account/full-split builders, development repository and restart/reload behavior, minimal HTTP/TCP parsing and response helpers, and stable server/dev-runtime/repository/browser-parity contract identities. Every fixture output labels its source and non-production posture.

The executable `serve` entrypoint is guarded by `trillionnium_world_fixture_runtime_policy_v1`. It requires the explicit `--allow-fixture-runtime` flag and accepts only literal IPv4/IPv6 loopback socket addresses with a nonzero port. Hostnames, wildcard addresses, public/private non-loopback addresses and implicit startup are rejected before a listener is created.

## State and invariants

One fenced World aggregate writer owns a revision at a time. Command success is persisted only after complete validation; rejection preserves prior state. The current JSON file repository is a development fixture, not a crash-durable store: it must never be selected by a production profile, and interruption or corruption receives no durability credit. Production state must use the catalog-verified PostgreSQL contract, stable IDs/hashes, epochs, immutable events, and no dual writer with legacy CEX mutation.

## Dependencies and boundaries

The server depends on World API/command/domain/map/projection/UI crates. Runtime adapters implement identity, repository, ledger, evidence, and metrics without leaking CEX/Nakama internals into domain packages. Network handlers cannot bypass application validation. Production routing, TLS, secrets, service identity, database pools, and observability are external deployment adapters.

## Fixture server invocation

The only accepted listener form is an explicit local-development invocation:

```bash
cargo run --locked --manifest-path trillionnium/crates/world-authority/Cargo.toml \
  -p trnm-world-server -- serve \
  --allow-fixture-runtime \
  --bind 127.0.0.1:8787
```

The machine-readable policy is available without opening a socket:

```bash
cargo run --locked --manifest-path trillionnium/crates/world-authority/Cargo.toml \
  -p trnm-world-server -- fixture-runtime-policy
```

Neither command grants deployment, public ingress or production authorization.

## Failure and recovery

Missing explicit opt-in, noncanonical bind strings, hostnames, port zero and every non-loopback address fail before bind. Malformed requests, unsupported routes/contracts, invalid commands, repository corruption, stale revision/epoch, adapter failure, and oversized/partial I/O must fail closed with bounded responses. Restart can exercise a previously written valid fixture; ambiguous, truncated or corrupt data is rejected. Production recovery uses PostgreSQL catalog/fencing/replay and exact reconciliation, never fixture acceptance.

## Testing and evidence

Tests cover explicit fixture opt-in, IPv4/IPv6 loopback admission, wildcard/public/hostname/whitespace/port-zero rejection, adapter separation, command/state parity, full-split responses, safe HTML, request behavior, file save/reload/restart, source provenance, no external path dependencies, and PostgreSQL cutover hostile matrices. The JSON fixture repository and hand-written TCP server remain development evidence only. Cross-host, TLS/public edge, identity, custody, endurance, backup/PITR, and human evidence remain open.

## Compatibility and change control

Server/API/repository/database/event versions are separate. Any route, mutation, persistence, epoch/fence, fixture, listener policy or adapter change requires contract tests, migration/rollback, threat/runbook updates, exact-head evidence, and review. Fixture code must not become an implicit production default. Weakening explicit opt-in or loopback-only binding is a security-sensitive breaking change.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-server-design.md`](../../../../docs/modules/world-authority/trnm-world-server-design.md).
