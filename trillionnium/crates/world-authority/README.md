# Trillionnium World-domain authority cutover workspace

Status: current convergence candidate; not part of the active eight-crate game-product workspace  
Owner: Trillionnium World domain-service maintainers  
Production authorization: `not_granted`

This bounded workspace restores the standalone **World-domain state and command boundary** that was historically embedded in CEX. It deliberately does not restore the retired Bevy client, legacy RTS product, release-packet machinery, or a second public online authority.

## Authority boundary

World owns deterministic World aggregates, domain command decisions, map/content lookup, rebuildable projections, internal versioned API contracts, and the standalone domain service. Nakama remains the sole target owner of canonical online admission, participant/session identity, global command/event ordering, reconnect generations, archive roots, and `MatchCompletedV1` signing. CEX remains authoritative for wallet/ledger state and custody. Chain and Integration retain finality and cross-repository release responsibilities.

The binding interpretation is `docs/adr/0003-world-domain-authority-and-nakama-canonical-online.md`. Neither this workspace nor its fixture server may expose a production player path that bypasses Nakama.

## Included crates

| Crate | Responsibility | Module contract | Detailed design |
|---|---|---|---|
| `trnm-world-domain` | deterministic World aggregate types and invariants | [`trnm-world-domain/README.md`](trnm-world-domain/README.md) | [`../../../docs/modules/world-authority/trnm-world-domain-design.md`](../../../docs/modules/world-authority/trnm-world-domain-design.md) |
| `trnm-world-command` | pure domain command validation and state transition | [`trnm-world-command/README.md`](trnm-world-command/README.md) | [`../../../docs/modules/world-authority/trnm-world-command-design.md`](../../../docs/modules/world-authority/trnm-world-command-design.md) |
| `trnm-world-projection` | rebuildable read projections and route artifacts | [`trnm-world-projection/README.md`](trnm-world-projection/README.md) | [`../../../docs/modules/world-authority/trnm-world-projection-design.md`](../../../docs/modules/world-authority/trnm-world-projection-design.md) |
| `trnm-world-map-provider` | versioned fixture map/content provider and map-pack gates | [`trnm-world-map-provider/README.md`](trnm-world-map-provider/README.md) | [`../../../docs/modules/world-authority/trnm-world-map-provider-design.md`](../../../docs/modules/world-authority/trnm-world-map-provider-design.md) |
| `trnm-world-ui-fragments` | escaped Rust-owned presentation fragments | [`trnm-world-ui-fragments/README.md`](trnm-world-ui-fragments/README.md) | [`../../../docs/modules/world-authority/trnm-world-ui-fragments-design.md`](../../../docs/modules/world-authority/trnm-world-ui-fragments-design.md) |
| `trnm-world-api` | internal DTO/trait boundary for domain adapters | [`trnm-world-api/README.md`](trnm-world-api/README.md) | [`../../../docs/modules/world-authority/trnm-world-api-design.md`](../../../docs/modules/world-authority/trnm-world-api-design.md) |
| `trnm-world-server` | fixture/file-backed standalone development and cutover smoke server | [`trnm-world-server/README.md`](trnm-world-server/README.md) | [`../../../docs/modules/world-authority/trnm-world-server-design.md`](../../../docs/modules/world-authority/trnm-world-server-design.md) |

## Qualification state

The current source is a cutover candidate. Fixture identity/session/account/ledger/evidence/metrics adapters and the file-backed repository are allowed only for deterministic parity, restart, recovery, and review. A production deployment must provide independently reviewed fail-closed identity/session, durable PostgreSQL repository, ledger, evidence, metrics, routing, and observability adapters, and must prove stable-ID/canonical-hash reconciliation with no dual writer.

## Validation

From the repository root:

```bash
bash scripts/check-trnm-world-authority-cutover.sh
python3 scripts/check-trnm-world-authority-documentation.py
bash scripts/run-trnm-world-authority-postgres-check.sh
```

The source gate verifies the exact seven-crate dependency closure, rejects sibling/external path dependencies, runs format/test/Clippy, and exercises adapter, split, restart/reload, provenance, and database contracts. A green source or PostgreSQL result does not grant hosted exact-head, protected-governance, cross-repository, deployment, custody, public-network, human, legal/commercial, or production evidence.

The source was recovered from Git commit `d44d8930c917b55da7b23eb19e9645feb8f4ee59` into a separate bounded workspace so the active First Contact and compatibility-server crates remain independently reviewable. Any semantic change requires a versioned contract, migration/rollback analysis, exact-head tests, and review from every affected authority owner.
