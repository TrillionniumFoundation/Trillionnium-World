---
status: current-candidate
owner: trillionnium-world-architecture
last_reviewed: 2026-09-08
review_due: 2026-10-08
implementation_conformance: not-implied
---

# World-domain authority workspace module designs

This index covers the isolated seven-crate `trillionnium/crates/world-authority` cutover workspace. It is separate from the active eight-crate game-product workspace and remains a candidate until exact source, PostgreSQL, repository governance, cross-repository reconciliation, deployment, and accountable external evidence close.

| Crate | Design |
|---|---|
| `trnm-world-domain` | [`trnm-world-domain-design.md`](trnm-world-domain-design.md) |
| `trnm-world-command` | [`trnm-world-command-design.md`](trnm-world-command-design.md) |
| `trnm-world-projection` | [`trnm-world-projection-design.md`](trnm-world-projection-design.md) |
| `trnm-world-map-provider` | [`trnm-world-map-provider-design.md`](trnm-world-map-provider-design.md) |
| `trnm-world-ui-fragments` | [`trnm-world-ui-fragments-design.md`](trnm-world-ui-fragments-design.md) |
| `trnm-world-api` | [`trnm-world-api-design.md`](trnm-world-api-design.md) |
| `trnm-world-server` | [`trnm-world-server-design.md`](trnm-world-server-design.md) |

Every design documents scope/authority, public interfaces, state/invariants, dependency direction, concurrency/cancellation, persistence/durability, idempotency/retry/failures, version/migration, resource/performance, observability/security, evidence traceability, and open work. ADR-0003 is binding: this workspace may own World-domain mutation after cutover but cannot become a second Nakama canonical online authority or a CEX custody service.
