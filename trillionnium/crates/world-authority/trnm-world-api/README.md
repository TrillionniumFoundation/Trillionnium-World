# trnm-world-api

Status: current cutover candidate; production adapter contracts defined, implementations absent  
Owner: Trillionnium World internal domain API

## Purpose

This crate defines versioned DTOs and adapter traits connecting deterministic World-domain commands/projections to bounded identity, session, account, repository, ledger, evidence, metrics, routing and deployment adapters.

## Authority and non-goals

The API is an internal cutover boundary, not a public player-authority contract. Trait presence does not prove a production adapter. Implementations must not bypass Nakama canonical online admission/order/recovery/signing, CEX ledger/custody, Chain finality, or Integration release locks. Fixture adapters are test-only, and `production_authorization` remains `not_granted`.

## Public contracts

The legacy `trillionnium_world_runtime_adapter_v1` surface supports deterministic fixture tests and includes home/command/route/map/tactics/full-split values plus `WorldIdentityAdapter`, `WorldSessionGuard`, `WorldAccountAdapter`, `WorldLedgerAdapter`, `WorldRepository`, `WorldEvidenceSink`, and `WorldMetricsSink`. Those synchronous, infallible trait seams are not a production implementation contract.

`trillionnium_world_production_adapter_v1` is the fail-closed production boundary. It adds stable machine error codes; credential references rather than secret values; immutable request, actor/account/session/generation, aggregate/revision/writer-epoch and command-hash context; repository load/lookup/commit contracts; ledger lookup-before-submit; immutable evidence; bounded metrics; Nakama/internal routing authorization; deployment identity and exact component-lock fields. Its readiness output explicitly reports every implementation and qualification as absent.

## State and invariants

DTOs bind API/domain/component versions, stable actor/account/session/aggregate/revision/receipt identifiers, accepted/rejected status, and source-of-truth. Repository commit, ledger effect, evidence append and metrics emission are separate outcomes; one cannot imply another. An exact duplicate may return a previously recorded receipt, while reuse of the same identity with altered bytes must fail. Fixture status remains explicitly fixture/non-production.

## Dependencies and boundaries

The crate depends on domain/command/projection/map types and serialization. It does not implement sockets, SQL, files, tokens, secret storage, canonical online ordering, retries or deployment. Production adapters live in owning runtime/repository packages and obey least privilege, transaction, fencing, timeout, lookup-before-submit, component-lock and observability contracts.

## Failure and recovery

The production contract returns `WorldProductionAdapterResult<T>` with stable codes for malformed input, unsupported contract, identity/audience/session failures, stale writer epoch/revision, identity conflict, repository conflict/ambiguity, ledger ambiguity/permanent rejection, routing, capacity, shutdown and internal integrity. `retryable` is explicit and safe diagnostics cannot carry credentials. Ambiguity after a possible commit requires exact lookup; callers never convert adapter uncertainty into local success.

## Testing and evidence

Tests cover serialization, contract/source binding, fixture labels, all legacy trait roles, explicit production-unready posture, stable machine errors and absence of raw secret-value fields. Production identity, PostgreSQL repository, CEX ledger, immutable evidence/metrics, routing, deployment and exact-head behavior still require independently reviewed implementations and black-box evidence.

A machine-readable snapshot is available through:

```bash
cargo run --locked --manifest-path trillionnium/crates/world-authority/Cargo.toml \
  -p trnm-world-server -- production-adapter-readiness
```

Its expected result remains `production_ready=false` until real implementations and evidence exist.

## Compatibility and change control

Changing a DTO, trait method, identity binding, error/retry rule, source-of-truth or capability role requires an API/adapter version, generated schema and vectors, old/new adapter conformance, rollout/rollback, component-lock update and affected-owner review. New public routes require a separate ADR and Nakama integration. A source-only capability declaration cannot set `implementation_available`, `exact_head_qualified` or production authorization to true.

## Detailed design

See [`../../../../docs/modules/world-authority/trnm-world-api-design.md`](../../../../docs/modules/world-authority/trnm-world-api-design.md).
