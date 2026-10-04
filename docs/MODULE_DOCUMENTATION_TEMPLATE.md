# `<module-name>`

Status: `<experimental|alpha|beta|stable|migration-only|mvp>`  
Owner: `<team or accountable maintainer>`  
Code path: `<repository-relative path>`  
Workspace/release lane: `<lane>`

## Purpose

Describe the single responsibility of the module and the outcome it enables.

## Responsibilities

- Behavior owned by this module.
- State or contracts for which it is authoritative.
- Outputs it may produce.

## Non-goals

- Adjacent behavior explicitly owned elsewhere.
- Claims this module must not be used to make.

## Dependencies and public contracts

Document direct dependencies, protocol versions, schemas, endpoints, events, and examples.

## State model and invariants

Describe states, legal transitions, idempotency, concurrency, ordering, determinism, limits, persistence, and recovery rules.

## Failure semantics

For each important failure class, document whether the operation retries, fails closed, rolls back, quarantines state, or requires operator intervention.

## Configuration

List every supported variable/file, type, default, secret classification, and invalid-value behavior. Never include real credentials.

## Build and test

```bash
# exact commands from repository root
```

Describe unit, integration, property, fuzz, compatibility, migration, fault-injection, and performance coverage.

## Compatibility and migrations

Define supported versions, upgrade order, downgrade/rollback rules, deprecation window, and data migration behavior.

## Security

State protected assets, trust assumptions, authentication/authorization, abuse controls, key handling, and threat-model references.

## Observability and operations

List logs, metrics, alerts, readiness signals, dashboards, runbooks, RTO/RPO, and evidence locations.

## Open gaps

Link every unresolved item to the active gap-closure board or a GitHub issue. Do not label a gap closed without required evidence.
