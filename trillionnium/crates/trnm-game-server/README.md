# `trnm-game-server`

Status: **alpha**  
Owner: Trillionnium Foundation game-product maintainers  
Workspace: `trillionnium/Cargo.toml`

## Purpose

Runs the PostgreSQL-backed dedicated campaign and realtime match authority, authenticated HTTP/WebSocket surfaces, durability journals, and settlement integration.

## Responsibilities

- Online campaign ownership, lobby/product operations, actor lifecycle, command ordering, snapshots, reconnect, and terminal settlement.
- Database schema migrations and transactional persistence.
- Published-tick and terminal/abandonment durability witnesses.
- Fleet fencing, readiness, rate/body limits, CEX and signer integration.

## Non-goals

The native client never becomes authoritative. The single-host journal is not cross-host RPO=0 or multi-region HA. Signing seed material must not live in the game-server process.

## Contracts and invariants

- `trnm_online_authority_v3` / `trnm-online-authority-2026.07-v3`.
- `trnm_online_production_v2` and `trnm-online-stream-v1`.
- PostgreSQL is mandatory; production has no in-memory fallback.
- One physical host owns one canonical journal root and fenced authority identity.
- Public state and receipts remain behind durable database/journal barriers.
- Uncertain durability poisons readiness and stops admission.
- Duplicate acknowledgement requires the full authority tuple to match.
- Cross-host active-match takeover remains blocked until replicated RPO=0 is proven.

Startup dependency, migration, fencing, journal, hash, cursor, or durability uncertainty fails closed. SIGTERM stops admission before bounded actor flush.

## Configuration

Production startup consumes variables including `DATABASE_URL`, `TRNM_CEX_LEDGER_URL`, `TRNM_GAME_AUTHORITY_TOKEN`, `TRNM_ENTITLEMENT_SIGNER_URL`, `TRNM_ENTITLEMENT_SIGNER_TOKEN`, `TRNM_MODERATOR_TOKEN`, fleet identity/region/endpoint/host/capacity values, `TRNM_GAME_SERVER_BIND_ADDR`, `TRNM_GAME_SERVER_TICK_MS`, `TRNM_ASSET_ROOT`, and `TRNM_PUBLISHED_TICK_JOURNAL_DIR`.

Tokens and database URLs are secrets. Journal paths must be canonical, permission-restricted, and covered by recovery procedures.

## Build and test

```bash
cargo test --manifest-path trillionnium/Cargo.toml -p trnm-game-server --all-targets --locked
cargo clippy --manifest-path trillionnium/Cargo.toml -p trnm-game-server --all-targets --locked -- -D warnings
```

Migrations require checksum/serialization, rolling-writer analysis, old-pair probes where advertised, interruption tests, rollback limits, and lineage evidence.

## Security

Protect sessions, admin/authority tokens, signer credentials, database access, journal paths, and fleet identity. Review SSRF, replay, privilege escalation, split-brain, amplification, and ambiguous commit.

## References

- [`trnm-online-authority-v3.md`](../../../docs/development/trnm-online-authority-v3.md)
- [`trnm-online-production-v2.md`](../../../docs/development/trnm-online-production-v2.md)
- [`trnm-native-game-release-gates-v1.md`](../../../docs/development/trnm-native-game-release-gates-v1.md)
- [`Gap closure board`](../../../docs/release/GAP_CLOSURE_BOARD_2026-09-11.md)

## Detailed design

- [`trnm-game-server detailed design`](../../../docs/modules/trnm-game-server-design.md)
