# trnm-world-api

Status: current cutover candidate  
Owner: Trillionnium World internal domain API

## Purpose

This crate defines versioned DTOs and adapter traits connecting deterministic World-domain commands/projections to bounded identity, session, account, repository, ledger, evidence, metrics, and server adapters.

## Authority and non-goals

The API is an internal cutover boundary, not a public player-authority contract. Trait presence does not prove a production adapter. Implementations must not bypass Nakama canonical online admission/order/recovery/signing, CEX ledger/custody, Chain finality, or Integration release locks. Fixture adapters are test-only.

## Public contracts

The surface includes home/command/route/map/tactics/full-split responses; actor/session/account/ledger/repository/evidence/metrics receipt/status values; `WorldIdentityAdapter`, `WorldSessionGuard`, `WorldAccountAdapter`, `WorldLedgerAdapter`, `WorldRepository`, `WorldEvidenceSink`, and `WorldMetricsSink`; and stable API/runtime-adapter/account contract identities.

## State and invariants

DTOs bind API/domain/component versions, stable actor/account/session/aggregate/revision/receipt identifiers, accepted/rejected status, and source-of-truth. Adapter calls cannot return an accepted result with missing/crossed identity. Repository and ledger operations are separate; a ledger receipt does not imply World apply, and World state does not imply a CEX effect. Fixture status remains explicitly fixture/non-production.

## Dependencies and boundaries

The crate depends on domain/command/projection/map types and serialization. It does not implement sockets, SQL, files, tokens, secret storage, canonical online ordering, or retries. Production adapters live in owning runtime/repository packages and obey least privilege, transaction, fencing, timeout, and observability contracts.

## Failure and recovery

Implementations return stable bounded decisions/receipts and fail closed on authentication, audience, version, revision, hash, lease, repository, ledger, or evidence errors. Ambiguous remote ledger results require exact lookup-before-submit outside World mutation. Callers do not convert a trait error into local success.

## Testing and evidence

Tests cover serialization, contract/source binding, fixture labels, all trait roles, identity/account/session separation, repository/ledger separation, malformed/crossed results, and mock/production adapter conformance suites. Production identity, PostgreSQL, CEX, metrics/evidence, and deployment behavior need black-box evidence.

## Compatibility and change control

Changing a DTO, trait method, identity binding, error/retry rule, or source-of-truth semantics requires an API version, adapter compatibility matrix, migration/rollback, fixture and production conformance updates, and affected-owner review. New public routes require a separate ADR and Nakama integration.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-api-design.md`](../../../../docs/modules/world-authority/trnm-world-api-design.md).
